trait Loggable {
    // Каждая структура, которая хочет быть Loggable, 
    // должна написать свою версию функции format_message
    fn format_message(&self) -> String;
}

struct User {
    username: String,
    warns: u8,
}

// Реализуем умение Loggable для Обычного Игрока
impl Loggable for User {
    fn format_message(&self) -> String {
        format!("👤 Игрок @{} | Предупреждения: {}", self.username, self.warns)
    }
}

struct Admin {
    nickname: String,
    level: u8,
}

// Реализуем умение Loggable для Админа
impl Loggable for Admin {
    fn format_message(&self) -> String {
        format!("🛡️ Модератор [{}] (Уровень: {})", self.nickname, self.level)
    }
}


// Эта функция говорит: "Я приму любой тип данных, который реализует типаж Loggable"
fn send_to_telegram(item: &impl Loggable) {
    let text = item.format_message(); // Rust точно знает, что этот метод есть!
    println!("Отправка в API Telegram: \n{}", text);
}

fn main() {
    let player = User { username: String::from("Magnus"), warns: 1 };
    let mod_user = Admin { nickname: String::from("Saturn_Admin"), level: 3 };

    // Кормим функции абсолютно разные структуры — и всё работает!
    send_to_telegram(&player);
    println!("-------------------");
    send_to_telegram(&mod_user);
}