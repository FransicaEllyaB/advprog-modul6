# Tutorial Pemrograman Lanjut
## Fransisca Ellya Bunaren - 2306152286

## Commit 1 Reflection
> what is inside the handle_connection

Fungsi `handle_connection` dalam Rust digunakan untuk menangani koneksi TCP dari klien dengan membaca permintaan HTTP yang dikirim. Fungsi ini membungkus koneksi stream dalam BufReader agar dapat membaca data secara lebih efisien. Selanjutnya, data dibaca baris per baris menggunakan `.lines()` dan dikumpulkan dalam sebuah vektor setelah difilter dengan .`take_while(|line| !line.is_empty())`, yang memastikan hanya header HTTP yang diambil hingga baris kosong ditemukan. Dengan cara ini, server dapat memperoleh informasi seperti metode HTTP (GET, POST), path URL, serta berbagai header lainnya seperti User-Agent dan Host. Data yang dikumpulkan ini kemudian dapat digunakan untuk memproses permintaan klien dan menentukan respons yang sesuai.

## Commit 2 Reflection
![Screenshot 2025-03-18 043912](https://github.com/user-attachments/assets/e32490e4-b6ab-4a61-b57e-b7e59cdb5dcd)

Fungsi handle_connection bertanggung jawab untuk menangani setiap koneksi yang masuk melalui TcpStream. Pertama, ia membuat BufReader untuk membaca data dari stream secara lebih efisien. Kemudian, fungsi ini membaca header permintaan HTTP baris per baris menggunakan .lines(), mengabaikan baris kosong yang menandai akhir dari header. Setelah itu, ia menyiapkan respons HTTP dengan status "200 OK", membaca isi file hello.html, dan menentukan panjangnya. Akhirnya, fungsi ini membentuk respons HTTP lengkap dengan header Content-Length dan mengirimkannya kembali ke klien menggunakan stream.write_all(), memastikan bahwa browser atau aplikasi klien menerima dan menampilkan konten HTML yang diminta.

