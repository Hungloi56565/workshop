Stellar Pay & Tokenize 

📝 Mô tả dự án
Ứng dụng web phi tập trung (dApp) xây dựng trên mạng Stellar Testnet. Dự án tập trung vào việc xử lý các giao dịch XLM cơ bản và đặt nền móng cho việc quản lý tài sản số thông qua hợp đồng thông minh Soroban. Đây là bài nộp chính thức cho Level 1 - White Belt Submission.

💡 Ý tưởng & Tầm nhìn (Project Ideas)
Dự án không chỉ dừng lại ở một ứng dụng ví đơn giản mà hướng tới các giải pháp tài chính thực tiễn:

-Simple Payment dApp: Một ứng dụng thanh toán tinh gọn cho phép chuyển XLM (Native Asset) đến bất kỳ địa chỉ ví nào trên mạng Testnet với tốc độ giao dịch nhanh và phí cực thấp.

-Token hóa tài sản (Asset Tokenization): Sử dụng Smart Contract (Soroban) để tạo ra các token tùy chỉnh. Ví dụ:

-Token hóa điểm thưởng: Chuyển đổi điểm tích lũy của khách hàng thành token trên blockchain để tăng tính thanh khoản và minh bạch.

-Hệ thống tín dụng nội bộ: Tạo ra các "Credit" để thanh toán trong các hệ thống đóng (như trường học hoặc doanh nghiệp).

-Split Bill Calculator: Tích hợp tính năng chia hóa đơn tự động và gửi thanh toán trực tiếp từ UI đến danh sách người nhận.

🚀 Tính năng chính

🔌 Wallet Connection: Kết nối và ngắt kết nối an toàn với Freighter Wallet.

💰 Real-time Balance: Tự động truy vấn và hiển thị số dư XLM (Native) ngay khi kết nối thành công.

💸 Native Payment: Luồng giao dịch chuyển XLM hoàn chỉnh với thông báo trạng thái trực quan.

📜 Transaction History: Hiển thị mã Hash giao dịch để người dùng có thể kiểm tra trên Stellar Expert.


Ứng dụng web cho phép chuyển XLM trên **Stellar Testnet** thông qua ví Freighter.

---
## Development Standards

- **UI Setup:** Xây dựng bằng HTML/CSS/JS thuần, sử dụng bố cục dạng thẻ (card-based layout) gọn gàng, chia rõ khu vực thao tác và khu vực hiển thị trạng thái giao dịch.
- **Wallet Integration:** Sử dụng thư viện `@stellar/freighter-api` qua CDN để xử lý việc kết nối (`requestAccess`), lấy địa chỉ ví (`getPublicKey`) và ký giao dịch (`signTransaction`). Có nút để ngắt kết nối và reset giao diện.
- **Balance Fetching:** Sử dụng `StellarSdk.Horizon.Server` kết nối tới Horizon Testnet, dùng hàm `loadAccount` để fetch `balances` và lọc lấy tài sản `native` (XLM) hiển thị trực tiếp lên UI.
- **Transaction Logic & Error Handling:** Sử dụng `TransactionBuilder` để tạo `Operation.payment` gửi native XLM. Toàn bộ luồng được bọc trong khối `try/catch`. Lỗi từ chối ký ví hoặc lỗi do không đủ số dư đều được catch và in ra box "Kết quả giao dịch" với màu đỏ (Error state).

## Contract
<img width="1919" height="963" alt="image" src="https://github.com/user-attachments/assets/9c24ac31-ce7c-4677-95e0-0f9b559c0be4" />

| Thông tin | Giá trị |
|-----------|---------|
| **Contract ID** | `CDH5AOFW6LNHJYR5QHZAIHKR4O3RE6HLZBPXU2VFSIHR7E6MLO6IORUE` |
| **Network** | Stellar Testnet |
| **Contract Type** | Stellar Asset Contract (SAC) — Native XLM |
| **WASM Hash** | `a62a16d0...` |

🔗 [Xem contract trên StellarChain Explorer](https://stellarchain.io/contracts/CDH5AOFW6LNHJYR5QHZAIHKR4O3RE6HLZBPXU2VFSIHR7E6MLO6IORUE)

### Ảnh contract trên Stellar Explorer

Link giao dịch mở trên Stellar Expert 

https://stellar.expert/explorer/testnet/account/GCFJGNXVFCKSMM4S6JEZMJQFJQDSKRLL563DVSOHAPGAE3NNP3PUMIXA

![Smart Contract trên StellarChain Explorer](./contract_screenshot.png)

---

## Giao dịch

| | |
|--|--|
| **Từ** | `GCFJGNXVFCKSMM4S6JEZMJQFJQDSKRLL563DVSOHAPGAE3NNP3PUMIXA` |
| **Đến** | `GCTC5QVPCSW2RRFC7LR6THXYLADP74TZVF6ISZOKI564TZB26FJSB36N` |
| **Số lượng** | 10 XLM |
| **Phí** | 0.00001 XLM |
| **Thời gian** | 2026-03-25 05:44:45 UTC |

🔗 [Xem lịch sử giao dịch trên Stellar Expert](https://stellar.expert/explorer/testnet/account/GCFJGNXVFCKSMM4S6JEZMJQFJQDSKRLL563DVSOHAPGAE3NNP3PUMIXA)

🧪 Kiểm thử (Testing)

Hợp đồng thông minh trong lib.rs đã được kiểm thử với ít nhất 3 trường hợp quan trọng:

-Initialize & Mint: Kiểm tra việc khởi tạo token và cấp phát số dư ban đầu cho người dùng.

-Successful Transfer: Đảm bảo luồng chuyển token giữa hai địa chỉ diễn ra chính xác.

-Insufficient Balance Error: Xác minh hệ thống bắt lỗi và ngăn chặn giao dịch khi người gửi không đủ số dư.


<img width="849" height="450" alt="Ảnh chụp màn hình 2026-04-27 210928" src="https://github.com/user-attachments/assets/3dbfa263-dadb-4251-8176-f2022a11b5ff" />


---

## Cách dùng

### Yêu cầu
- Trình duyệt có cài extension [Freighter Wallet](https://www.freighter.app/)
- Ví đã có XLM testnet (nhận miễn phí tại [Friendbot](https://laboratory.stellar.org/#account-creator?network=test))

### Chạy ứng dụng
1. Mở file `frontend.html` trên trình duyệt
2. Nhấn **"Kết nối Ví"** — Freighter sẽ hiện popup xác nhận
3. Nhập **địa chỉ nhận** (`G...`, 56 ký tự) và **số lượng XLM**
4. Nhấn **"Thực hiện hành động"** — ký transaction trong Freighter
5. Kết quả hiển thị ở ô "Kết quả giao dịch"

| Stellar SDK | `stellar-sdk` v12.3.0 (CDN) |
| Ví | Freighter API v2.0.0 (CDN) |
| Network | Stellar Testnet — Horizon `https://horizon-testnet.stellar.org` |# workshop
