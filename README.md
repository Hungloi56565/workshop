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
