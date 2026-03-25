<img width="1919" height="963" alt="image" src="https://github.com/user-attachments/assets/ad698c51-8ccc-406e-8064-66b87cfb9125" /># Stellar XLM Transfer DApp

Ứng dụng web cho phép chuyển XLM trên **Stellar Testnet** thông qua ví Freighter.

---

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
