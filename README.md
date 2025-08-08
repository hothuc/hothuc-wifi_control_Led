# ESP32-C3 Wi-Fi Access Point & LED Control Web Server

Dự án này biến **ESP32-C3** thành một **Wi-Fi Access Point (AP)** kèm **web server** để điều khiển LED qua trình duyệt.  
Khi có thiết bị kết nối hoặc rời khỏi AP, LED sẽ thay đổi trạng thái tự động và thông báo sẽ hiển thị trên terminal.

## 📌 Tính năng
- **Tạo Access Point riêng** với SSID và mật khẩu định sẵn.
- **Theo dõi sự kiện kết nối Wi-Fi**:
  - Khi thiết bị kết nối → LED bật (`ON`).
  - Khi thiết bị rời đi → LED tắt (`OFF`).
- **Web interface đơn giản**:
  - Xem trạng thái LED.
  - Điều khiển LED bật/tắt qua nút bấm.
- **Cập nhật trạng thái thời gian thực** trên trang web (qua `fetch()` mà không cần reload trang).
- **Giao diện HTML/CSS gọn nhẹ**, tương thích điện thoại và máy tính.

## 🛠 Phần cứng yêu cầu
- [ESP32-C3 Core Board LuatOS.](https://wiki.luatos.org/chips/esp32c3/board.html)
- LED onboard điều khiển bằng GPIO12.

## Quy trình thực hiện
- Về việc cài đặt Rust và môi trường thì bạn hãy đọc trong cuốn sánh này [The Rust on ESP Book](https://docs.espressif.com/projects/rust/book/)
- Bắt đầy bằng việc Generating Projects from Templates ở đây tôi dùng std nên:
 - Cài đặt cargo generate:
    cargo install cargo-generate
 - Tải template:
     cargo generate esp-rs/esp-idf-template cargo
Lưu ý: Khi thực hiện thì máy sẽ hỏi tên project bạn nhập tên mình muốn vào, sau đó là lựa chọn board phù hợp, cuối cùng là cài đặt cá nhân nếu mà bạn muốn để mặc định thì chọn false ở đây tôi để mặc định nên chọn false.
- Tiếp theo chúng ta sẽ chọn những thư viện sẽ dùng và bỏ vào file Cargo.toml ở đây tôi dùng:
    esp-idf-hal = "0.45"
    esp-idf-sys = "0.36"
    anyhow = "1"
    heapless = "0.8"
chú ý: các thư viện rất dễ xung đột nên bạn cần kiểm tra các thư viện cẩn thận.
- Bước tiếp bạn thưc hiện code trong main.rs chú ý các giá trị:
|  Name  |           Value              |
|:-------|:-----------------------------|
|ssid    |SSID of your WiFi access point|
|password|Your WiFi password            |
- Cuối cùng chúng ta thực hiện build và flash xuống esp32c3 bằng lệnh:
    cargo run --release

## Thực hiện kết nối thực tiễn
- Bạn sử dụng thiết bị là điện thoại hay máy tính kết nối wifi với esp32c3 ở đây tôi đặt tên wifi là **ESP32_AP** và mật khẩu là **12345678** (lưu ý khi đặt mật khẩu bạn phải có 8 kí tự)
- Vào trình duyệt web truy cập vào hiện tại là đang IP động được ESP thông báo