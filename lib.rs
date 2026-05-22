#![no_std]
// Thêm BytesN vào danh sách import để hàm upgrade không bị lỗi biên dịch
use soroban_sdk::{contract, contracterror, contractimpl, contracttype, Address, Env, BytesN};

// Thời gian sống của dữ liệu trên Stellar (1 ledger ~ 5s, 17280 ledgers ~ 1 ngày)
const DAY_IN_LEDGERS: u32 = 17280;

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataKey {
    Debt(Address, Address),
    Admin, // Thêm khóa này để lưu trữ địa chỉ của Admin [cite: 962]
}

// Định nghĩa các mã lỗi
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    NegativeAmount = 1,
    InsufficientDebt = 2,
    AlreadyInitialized = 3, // Mã lỗi nếu cố tình gọi hàm khởi tạo 2 lần
}

#[contract]
pub struct IouContract;

#[contractimpl]
impl IouContract {
    /// HÀM KHỞI TẠO (CHỈ GỌI 1 LẦN DUY NHẤT NGAY SAU KHI DEPLOY)
    /// Hàm này sẽ thiết lập ai là Admin của hợp đồng.
    pub fn initialize(env: Env, admin: Address) -> Result<(), Error> {
        // Kiểm tra xem hợp đồng đã được khởi tạo trước đó chưa
        if env.storage().instance().has(&DataKey::Admin) {
            return Err(Error::AlreadyInitialized);
        }
        
        // Lưu địa chỉ Admin vào Instance Storage (bộ nhớ gọn nhẹ đi kèm hợp đồng) [cite: 964, 1053]
        env.storage().instance().set(&DataKey::Admin, &admin);
        
        // Gia hạn thời gian sống cho cấu hình admin này [cite: 966]
        env.storage().instance().extend_ttl(30 * DAY_IN_LEDGERS, 31 * DAY_IN_LEDGERS);
        
        Ok(())
    }

    /// 1. GHI NHẬN NỢ
    pub fn add_debt(
        env: Env,
        debtor: Address,
        creditor: Address,
        amount: i128,
    ) -> Result<(), Error> {
        debtor.require_auth();

        if amount <= 0 {
            return Err(Error::NegativeAmount);
        }

        let key = DataKey::Debt(debtor.clone(), creditor.clone());
        let current_debt: i128 = env.storage().persistent().get(&key).unwrap_or(0);

        env.storage().persistent().set(&key, &(current_debt + amount));
        env.storage().persistent().extend_ttl(&key, 30 * DAY_IN_LEDGERS, 31 * DAY_IN_LEDGERS);

        Ok(())
    }

    /// 2. THANH TOÁN / XÓA NỢ TRỰC TIẾP
    pub fn clear_debt(
        env: Env,
        creditor: Address,
        debtor: Address,
        amount: i128,
    ) -> Result<(), Error> {
        creditor.require_auth();

        if amount <= 0 {
            return Err(Error::NegativeAmount);
        }

        let key = DataKey::Debt(debtor.clone(), creditor.clone());
        let current_debt: i128 = env.storage().persistent().get(&key).unwrap_or(0);

        if current_debt < amount {
            return Err(Error::InsufficientDebt);
        }

        env.storage().persistent().set(&key, &(current_debt - amount));
        env.storage().persistent().extend_ttl(&key, 30 * DAY_IN_LEDGERS, 31 * DAY_IN_LEDGERS);

        Ok(())
    }

    /// 3. TRIỆT TIÊU NỢ BẮC CẦU (DEBT NETTING)
    pub fn net_debt(
        env: Env,
        creditor_c: Address,
        middleman_b: Address,
        debtor_a: Address,
        amount: i128,
    ) -> Result<(), Error> {
        creditor_c.require_auth();

        if amount <= 0 {
            return Err(Error::NegativeAmount);
        }

        let key_a_b = DataKey::Debt(debtor_a.clone(), middleman_b.clone());
        let key_b_c = DataKey::Debt(middleman_b.clone(), creditor_c.clone());

        let debt_a_b: i128 = env.storage().persistent().get(&key_a_b).unwrap_or(0);
        let debt_b_c: i128 = env.storage().persistent().get(&key_b_c).unwrap_or(0);

        if debt_a_b < amount || debt_b_c < amount {
            return Err(Error::InsufficientDebt);
        }

        env.storage().persistent().set(&key_a_b, &(debt_a_b - amount));
        env.storage().persistent().set(&key_b_c, &(debt_b_c - amount));

        env.storage().persistent().extend_ttl(&key_a_b, 30 * DAY_IN_LEDGERS, 31 * DAY_IN_LEDGERS);
        env.storage().persistent().extend_ttl(&key_b_c, 30 * DAY_IN_LEDGERS, 31 * DAY_IN_LEDGERS);

        Ok(())
    }

    /// 4. TRUY VẤN NỢ (HÀM CHỈ ĐỌC)
    pub fn get_debt(env: Env, debtor: Address, creditor: Address) -> i128 {
        let key = DataKey::Debt(debtor, creditor);
        env.storage().persistent().get(&key).unwrap_or(0)
    }

    /// HÀM NÂNG CẤP HỢP ĐỒNG
    pub fn upgrade(env: Env, new_wasm_hash: BytesN<32>) {
        // Đọc địa chỉ Admin đã lưu trong instance lưu trữ ra để đối chiếu
        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        
        // Bắt buộc phải có chữ ký của đúng Admin đó thì mới cho chạy tiếp 
        admin.require_auth();

        // Tiến hành cập nhật logic
        env.deployer().update_current_contract_wasm(new_wasm_hash);
    }
}