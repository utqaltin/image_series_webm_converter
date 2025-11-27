# 🖼️ Image Series Converter – `render_cli`

Küçük, etkileşimli bir CLI (Komut Satırı Arayüzü) aracıdır. PNG görsel serilerini `ffmpeg` kullanarak **WEBM / MP4 / GIF** formatlarına dönüştürür.

Unreal/Blender gibi, numaralandırılmış PNG karelerini (isteğe bağlı alfa kanalı ile) dışa aktardığınız ve web, Tauri uygulamaları vb. için şeffaf veya kırpılmış bir videoya ihtiyaç duyduğunuz iş akışları için tasarlanmıştır.

> 💡 Araç kasıtlı olarak **etkileşimlidir**: Karelerin bulunduğu bir klasörde çalıştırırsınız ve size birkaç soru sorar (ön ek, son ek, FPS, format, kırpma…), son `ffmpeg` komutunu yazdırır, çalıştırır ve ardından çıktıya ait meta verileri gösterir. Uzun `ffmpeg` argümanlarını hatırlamaya gerek kalmaz.

---

## Özellikler

- ✅ PNG kare serilerini şunlara dönüştürür:
    - **WEBM (VP9)** – şeffaflığı (alfa) destekler
    - **MP4 (H.264)** – standart video, alfa desteği yok
    - **GIF** – `palettegen` / `paletteuse` ile 8-bit palet oluşturur
- ✅ **Ön ek + 4 haneli kare numarası + son ek** desenlerini kabul eder, örn:
    - `S_Film.0000_5p.png`, `S_Film.0001_5p.png`, …
- ✅ WEBM/GIF için isteğe bağlı **şeffaflık** desteği
- ✅ `w:h:x:y` formatında isteğe bağlı **kırpma** (örn. `640:520:360:170`)
- ✅ Çalıştırmadan önce bir **özet** gösterir ve onay ister
- ✅ Çalıştırdığı tam `ffmpeg` komutunu yazdırır (yeniden kullanım veya ince ayar için kolaylık sağlar)
- ✅ Kodlamadan sonra, çıktı **meta verilerini** göstermek için `ffmpeg -i` komutunu çalıştırır
- ✅ Tek bir oturumda döngü yapabilir ve birden fazla seriyi işleyebilir

---

## Gereksinimler

### 1. ffmpeg

Bu araç, `ffmpeg`'in yalnızca bir sarmalayıcısıdır (wrapper), bu nedenle **mutlaka yüklü olmalıdır**.

İki seçeneğiniz var:

- **Global Kurulum** (Önerilen)

  `ffmpeg`'in `PATH`'inizde bulunabildiğinden emin olun.

  - **Windows (winget):**

    ```powershell
    winget install ffmpeg
    ```

  - **macOS (Homebrew):**

    ```bash
    brew install ffmpeg
    ```

  - **Debian/Ubuntu:**

    ```bash
    sudo apt-get install ffmpeg
    ```
    
  - **Arch:**

    ```bash
    sudo pacman -S ffmpeg
    ```

  - **Fedora:**

    ```bash
    sudo dnf install ffmpeg
    ```
    

- **Yerel İkili Dosya** (Local Binary)

  `ffmpeg.exe` dosyasını `render_cli.exe` ile **aynı klasöre** yerleştirin.

İndirme sayfası: <https://ffmpeg.org/download.html>

---

### 2. Rust Toolchain (Yalnızca kaynaktan derleme için)

Projeyi kendiniz derlemek isterseniz:

- <https://rustup.rs> aracılığıyla Rust'ı kurun
- Kurulumu doğrulayın:

  ```bash
  rustc --version
  cargo --version
