# Tutorial Pemrograman Lanjut
## Fransisca Ellya Bunaren - 2306152286

## Commit 1  reflection notes
> what is inside the handle_connection

Fungsi `handle_connection` dalam Rust digunakan untuk menangani koneksi TCP dari klien dengan membaca permintaan HTTP yang dikirim. Fungsi ini membungkus koneksi stream dalam BufReader agar dapat membaca data secara lebih efisien. Selanjutnya, data dibaca baris per baris menggunakan `.lines()` dan dikumpulkan dalam sebuah vektor setelah difilter dengan .`take_while(|line| !line.is_empty())`, yang memastikan hanya header HTTP yang diambil hingga baris kosong ditemukan. Dengan cara ini, server dapat memperoleh informasi seperti metode HTTP (GET, POST), path URL, serta berbagai header lainnya seperti User-Agent dan Host. Data yang dikumpulkan ini kemudian dapat digunakan untuk memproses permintaan klien dan menentukan respons yang sesuai.

## Commit 2  reflection notes
![Screenshot 2025-03-18 043912](https://github.com/user-attachments/assets/e32490e4-b6ab-4a61-b57e-b7e59cdb5dcd)

Fungsi handle_connection bertanggung jawab untuk menangani setiap koneksi yang masuk melalui TcpStream. Pertama, ia membuat BufReader untuk membaca data dari stream secara lebih efisien. Kemudian, fungsi ini membaca header permintaan HTTP baris per baris menggunakan .lines(), mengabaikan baris kosong yang menandai akhir dari header. Setelah itu, ia menyiapkan respons HTTP dengan status "200 OK", membaca isi file hello.html, dan menentukan panjangnya. Akhirnya, fungsi ini membentuk respons HTTP lengkap dengan header Content-Length dan mengirimkannya kembali ke klien menggunakan stream.write_all(), memastikan bahwa browser atau aplikasi klien menerima dan menampilkan konten HTML yang diminta.

## Commit 3  reflection notes
![Screenshot 2025-03-19 180136](https://github.com/user-attachments/assets/b7fc0c52-4442-459f-9747-4e54c546e903)

Server sekarang menangani berbagai jalur URL dengan memeriksa request_line dari permintaan HTTP yang masuk. Jika baris permintaan cocok dengan "GET / HTTP/1.1", server akan merespons dengan status: `HTTP/1.1 200 OK` dan konten `hello.html`. Selain itu, jika permintaan mengarah ke jalur lain yang tidak dikenali, server akan merespons dengan Status `HTTP/1.1 404 NOT FOUND` dan konten `404.html` . Saya menerapkan factoring dengan memisahkan pembuatan respon ke dalam fungsi terpisah `build_response`, Fungsi `handle_connection` sekarang fokus pada penentuan respons apa yang akan dikirim, sementara `build_response` menangani bagaimana memformat respons tersebut.  Refactoring diperlukan agar
* Keterbacaan (Readability): Kode yang sudah di-refactor lebih mudah dibaca dan dipahami, karena setiap bagian kode memiliki fungsi yang jelas dan terfokus.
* Kemampuan Diperluas (Extensibility): Kode yang di-refactor lebih mudah diperluas. Misalnya, jika ingin menambahkan rute baru, kita hanya perlu menambahkan kondisi baru tanpa mengubah logika pembuatan respons.
* Skalabilitas (Scalability): Seiring pertumbuhan kompleksitas web server, memiliki kode yang terstruktur dengan baik dan modular menjadi semakin penting. Refactoring meletakkan dasar untuk arsitektur yang lebih skalabel.
* Penanganan Kesalahan (Error Handling): Meskipun belum diimplementasikan sepenuhnya, struktur yang di-refactor memudahkan untuk menambahkan penanganan kesalahan yang tepat di masa depan.

## Commit 4 reflection notes
Ketika kode sudah dimodifikasi, jika membuka `127.0.0.1:7878/sleep` di satu jendela browser terlebih dahulu, permintaan tersebut akan membuat server `tidur` selama 10 detik sebelum memberikan respons. Selama permintaan pertama masih diproses jika mencoba membuka `127.0.0.1:7878` di jendela browser lain, permintaan kedua akan tertunda dan tidak diproses hingga permintaan pertama selesai. Setelah periode 10 detik berlalu, server akan merespons permintaan pertama lalu langsung menangani permintaan kedua dan memberikan respons. Eksperimen ini menunjukkan keterbatasan utama dari server yang berjalan dalam mode single-threaded, yaitu hanya bisa menangani satu permintaan dalam satu waktu. Jika ada permintaan yang lambat, seperti `127.0.0.1:7878/sleep`, semua permintaan lain terblokir hingga permintaan tersebut selesai diproses.

## Commit 5 reflection notes
Pada tahap ini kita mengubah web server dari single-threaded menjadi multi-threaded. `ThreadPool` bekerja dengan cara membuat sejumlah thread pekerja (`Worker`) yang siap menangani tugas secara bersamaan. Saat server menerima permintaan, tugas tersebut dikirim melalui kanal (`mpsc::channel`) ke thread pekerja yang tersedia. Setiap `Worker` akan menjalankan loop yang terus-menerus menunggu tugas baru, dan ketika ada tugas yang diterima, worker tersebut akan mengeksekusinya di thread terpisah. Dengan cara ini, beberapa permintaan dapat diproses secara konkuren tanpa harus menunggu satu per satu. Penggunaan `Arc<Mutex<T>>` memungkinkan `receiver` dibagikan ke banyak thread dengan aman, memastikan tidak ada dua worker yang mengambil tugas yang sama pada waktu bersamaan. Dengan implementasi ini, server menjadi lebih efisien dan tidak lagi terblokir oleh satu permintaan yang berjalan lama, mengatasi keterbatasan dari server yang hanya menggunakan satu thread.

## Commit bonus reflection notes
Perbandingan metode `new` dan `build`, yaitu metode `new` menggunakan `assert!` yang menyebabkan panic jika kondisi tidak terpenuhi, langsung mengembalikan objek ThreadPoll, dan tidak memberi kesempatan bagi pemanggil untuk menangani error dengan baik, sedangkan metode `build` mengembalikan `Result<ThreadPool, String>`, memberi pemanggil kontrol penuh untuk menangani error, dan mengikuti error dengan lebih aman. Penggunaan metode `build` memiliki beberapa keuntungan, yaitu eksplisit dalam penanganan error, tidak ada panic yang tidak terduga, API yang lebih konsisten, dan lebih sesuai dengan filosofi rust karena Rust mendorong penanganan error yang eksplisit.
