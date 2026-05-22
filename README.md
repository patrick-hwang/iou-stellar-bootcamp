# [cite_start]I Owe You - Decentralized Debt Tracker [cite: 304, 1144]

## [cite_start]Problem [cite: 1144]
Sinh viên thường có nhiều khoản nợ vi mô chéo nhau (tiền ăn, trà sữa...) nhưng việc ghi chép thủ công dễ gây nhầm lẫn, thất lạc. Việc sử dụng các ứng dụng ghi chú thông thường thì không đảm bảo sự đồng thuận và tính minh bạch giữa các bên (dễ bị sửa đổi hoặc xóa file).

## [cite_start]Solution [cite: 1144]
Xây dựng một hệ thống IOU (I Owe You) phi tập trung trên blockchain. Mọi khoản nợ được lưu on-chain đảm bảo tính minh bạch, dữ liệu không thể bị đơn phương chỉnh sửa hoặc xóa bỏ nếu không có chữ ký xác thực của các bên liên quan, tạo niềm tin tuyệt đối khi đối soát.

## [cite_start]Why Stellar [cite: 1144]
Nếu triển khai trên các blockchain khác (như Ethereum), việc liên tục cập nhật trạng thái nợ có thể tốn từ vài đô đến hàng chục đô la phí gas cho mỗi thao tác và mất vài phút để xác nhận. Trong tài chính truyền thống, việc đối trừ nợ bắc cầu (A nợ B, B nợ C nên A trả thẳng C) đòi hỏi người dùng phải tự tính toán thủ công hoặc qua trung gian ngân hàng tốn nhiều ngày rườm rà. 

Với Stellar, mọi thao tác xử lý logic hợp đồng này chỉ mất ~5 giây với mức phí gần như miễn phí (khoảng $0.000003). Tốc độ và chi phí này cực kỳ hoàn hảo để người dùng ghi nhận hoặc thanh toán các khoản nợ vi mô lẻ tẻ hàng ngày mà không bị "lỗ" tiền phí.

## [cite_start]Target User [cite: 1144]
Sinh viên và những người có nhu cầu quản lý các khoản chi tiêu/vay mượn nhóm.

## [cite_start]Live Demo [cite: 1144]
- [cite_start]**Network:** Stellar Testnet [cite: 1144]
- [cite_start]**Contract ID:** `CDCLLEXWCKPNXN62UU2XKJZOVZSJJSYC4ECACWSXMKC3XD43ENZNFKEK` [cite: 1144]
- [cite_start]**Transaction Link:** [Dán đường link từ Stellar Expert của bạn vào đây] [cite: 1132, 1144]

![Contract Screenshot](contract-detail.png) 
[cite_start]*(Bạn nhớ chụp màn hình giao dịch trên Stellar Expert lưu thành file contract-detail.png rồi để chung thư mục nhé)* [cite: 306, 365]

## [cite_start]Tech Stack [cite: 1145]
- [cite_start]**Smart Contract:** Rust / Soroban SDK v22 [cite: 1145]
- [cite_start]**Network:** Stellar Testnet [cite: 1145]

## [cite_start]Team [cite: 1145]
- [cite_start]Name: [Tên của bạn] [cite: 1146]
- [cite_start]Email: [Email của bạn] [cite: 1146]