use std:: io;
fn main () {
  // io
  // i - input
  // o - output
  // квадратное уравнения
  // ax^2 + bx + c = 0
  // D = b^2 - 4(a * c)
  let mut a_str = String::new();
  let mut b_str = String::new();
  let mut c_str = String::new();

  loop {
    println!("Решить квадратное уравнение");
    println!("Введите a");
    match io::stdin().read_line(&mut a_str) {
      Ok(_) => {},
      Err(e) => println!("ОШИБКА ВВОДА - {}", e)
    }
    println!("Введите b");
    match io::stdin().read_line(&mut b_str) {
      Ok(_) => {},
      Err(e) => println!("ОШИБКА ВВОДА - {}", e)
    }
    println!("Введите c");
    match io::stdin().read_line(&mut c_str) {
      Ok(_) => {},
      Err(e) => println!("ОШИБКА ВВОДА - {}", e)
    }

    let a: f64 = a_str.trim().parse().unwrap();
    let b: f64 = b_str.trim().parse().unwrap();
    let c: f64 = c_str.trim().parse().unwrap();


    let d: f64 = (b * b) - 4.0 * (a * c);

    if d > 0.0 {
      let x1 = ((-b) + d.sqrt()) / (2.0 * a);
      let x2 = ((-b) - d.sqrt()) / (2.0 * a);
      println!("Решено\n есть два корня\nD = {}\n Корень 1 ={}\nКорень 2 = {}", d, x1, x2);
    }

    if d == 0.0 {
      let x = (-b) / (2.0 * a);
      println!("Решено\n есть 1 корня\nD = 0\n Корень = {}", x);
    }
    if d < 0.0 {
      print!("Корней не существует!\nD < 0\n = {}", d);
    }

  }
}
// fn main() {
//         // io
//         // i - input
//         // o - output
//     let mut name = String::new();
//
//     println!("Введите свое имя");
//
//     match io::stdin().read_line(&mut name) {
//         Ok(_) => {
//             println!("Здравствуйте {} ", name );
//
//         },
//         Err(e) => {
//             println!("ОШИБка ПРОГРАММЫ - {}", e);
//         }
//     }
// }
