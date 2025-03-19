# Tutorial Pemrograman Lanjut
## Fransisca Ellya Bunaren - 2306152286

## Commit 1 Reflection
> what is inside the handle_connection

Fungsi `handle_connection` dalam Rust digunakan untuk menangani koneksi TCP dari klien dengan membaca permintaan HTTP yang dikirim. Fungsi ini membungkus koneksi stream dalam BufReader agar dapat membaca data secara lebih efisien. Selanjutnya, data dibaca baris per baris menggunakan `.lines()` dan dikumpulkan dalam sebuah vektor setelah difilter dengan .`take_while(|line| !line.is_empty())`, yang memastikan hanya header HTTP yang diambil hingga baris kosong ditemukan. Dengan cara ini, server dapat memperoleh informasi seperti metode HTTP (GET, POST), path URL, serta berbagai header lainnya seperti User-Agent dan Host. Data yang dikumpulkan ini kemudian dapat digunakan untuk memproses permintaan klien dan menentukan respons yang sesuai.

## Commit 2 Reflection
![Screenshot 2025-03-18 043912](https://github.com/user-attachments/assets/e32490e4-b6ab-4a61-b57e-b7e59cdb5dcd)

Fungsi handle_connection bertanggung jawab untuk menangani setiap koneksi yang masuk melalui TcpStream. Pertama, ia membuat BufReader untuk membaca data dari stream secara lebih efisien. Kemudian, fungsi ini membaca header permintaan HTTP baris per baris menggunakan .lines(), mengabaikan baris kosong yang menandai akhir dari header. Setelah itu, ia menyiapkan respons HTTP dengan status "200 OK", membaca isi file hello.html, dan menentukan panjangnya. Akhirnya, fungsi ini membentuk respons HTTP lengkap dengan header Content-Length dan mengirimkannya kembali ke klien menggunakan stream.write_all(), memastikan bahwa browser atau aplikasi klien menerima dan menampilkan konten HTML yang diminta.

## Commit 3 Reflection
![{A24B4483-744F-4B0F-A05B-44C2E2AC31AE}](https://github.com/user-attachments/assets/e3909830-102e-4558-a5a0-e6540908ed01)

Server sekarang menangani berbagai jalur URL dengan memeriksa request_line dari permintaan HTTP yang masuk. Jika baris permintaan cocok dengan "GET / HTTP/1.1", server akan merespons dengan status: `HTTP/1.1 200 OK` dan konten `hello.html`. Selain itu, jika permintaan mengarah ke jalur lain yang tidak dikenali, server akan merespons dengan Status `HTTP/1.1 404 NOT FOUND` dan konten `404.html` . Saya menerapkan factoring dengan memisahkan pembuatan respon ke dalam fungsi terpisah `build_response`, Fungsi `handle_connection` sekarang fokus pada penentuan respons apa yang akan dikirim, sementara `build_response` menangani bagaimana memformat respons tersebut.  Refactoring diperlukan agar
* Keterbacaan (Readability): Kode yang sudah di-refactor lebih mudah dibaca dan dipahami, karena setiap bagian kode memiliki fungsi yang jelas dan terfokus.
* Kemampuan Diperluas (Extensibility): Kode yang di-refactor lebih mudah diperluas. Misalnya, jika ingin menambahkan rute baru, kita hanya perlu menambahkan kondisi baru tanpa mengubah logika pembuatan respons.
* Skalabilitas (Scalability): Seiring pertumbuhan kompleksitas web server, memiliki kode yang terstruktur dengan baik dan modular menjadi semakin penting. Refactoring meletakkan dasar untuk arsitektur yang lebih skalabel.
* Penanganan Kesalahan (Error Handling): Meskipun belum diimplementasikan sepenuhnya, struktur yang di-refactor memudahkan untuk menambahkan penanganan kesalahan yang tepat di masa depan.