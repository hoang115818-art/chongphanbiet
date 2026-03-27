# chongphanbiet
<img width="1689" height="320" alt="image" src="https://github.com/user-attachments/assets/63e05559-dd05-415b-95a8-5244d9a9ddee" />

UnityContract – Stellar Smart Contract (Soroban)
1. Giới thiệu

UnityContract là một Stellar Smart Contract được xây dựng bằng Rust sử dụng Soroban SDK.
Mục tiêu của contract là khuyến khích tinh thần đoàn kết, bình đẳng và chống phân biệt vùng miền thông qua cơ chế “cam kết” on-chain.

Mỗi người dùng có thể gửi một cam kết xác thực (authenticated commitment), và hệ thống sẽ ghi nhận tổng số người đã tham gia.

Contract đảm bảo:

Mỗi địa chỉ chỉ được cam kết 1 lần
Chỉ admin mới được thay đổi thông điệp chính thức
Dữ liệu được lưu trữ minh bạch trên blockchain Stellar
2. Công nghệ sử dụng
Blockchain: Stellar
Smart Contract Platform: Soroban
Ngôn ngữ: Rust
SDK: soroban-sdk
Môi trường: no_std
3. Kiến trúc Contract
3.1 Storage Keys

Contract sử dụng enum DataKey để quản lý storage:

Admin – Địa chỉ quản trị
Total – Tổng số người đã cam kết
Message – Thông điệp đoàn kết hiện tại
User(Address) – Trạng thái cam kết của từng người dùng
4. Các chức năng chính
4.1 initialize(admin, message)

Khởi tạo contract.

Thiết lập admin
Thiết lập tổng cam kết = 0
Thiết lập thông điệp ban đầu
Chỉ được gọi 1 lần
4.2 commit(user)

Cho phép người dùng cam kết.

Yêu cầu xác thực require_auth()
Nếu user chưa từng cam kết:
Lưu trạng thái
Tăng tổng số cam kết lên 1
Nếu đã cam kết → không tăng lại
4.3 get_total()

Trả về tổng số người đã cam kết.

4.4 set_message(admin, message)

Cập nhật thông điệp.

Yêu cầu admin xác thực
Kiểm tra quyền admin
Cập nhật nội dung message
4.5 get_message()

Trả về thông điệp hiện tại.
