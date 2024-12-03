use std::fs::{OpenOptions, File};
use std::io::{self, BufRead, BufReader, Write, BufWriter};

fn login() -> Option<String> {
    print!("Введіть логін: ");
    let _ = io::Write::flush(&mut io::stdout());
    let mut username = String::new();
    io::stdin()
        .read_line(&mut username)
        .expect("Не вдалося зчитати ввід");
    let username = username.trim();

    print!("Введіть пароль: ");
    let _ = io::Write::flush(&mut io::stdout());
    let mut password = String::new();
    io::stdin()
        .read_line(&mut password)
        .expect("Не вдалося зчитати ввід");
    let password = password.trim();

    let file_path = "src/users.csv";

    // Відкриваємо файл та читаємо його вміст
    let file = File::open(file_path).ok()?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.ok()?;
        let columns: Vec<&str> = line.split(',').collect();

        // Перевіряємо, чи рядок має хоча б 3 колонки
        if columns.len() >= 3 {
            let file_username = columns[1].trim();
            let file_password = columns[2].trim();

            // Перевіряємо логін і пароль
            if file_username == username && file_password == password {
                return Some(file_username.to_string());
            }
        }
    }

    return None
}

fn register() -> Option<String> {
    // Prompt for a username
    print!("Введіть логін (мінімум 8 символів): ");
    let _ = io::Write::flush(&mut io::stdout());
    let mut username = String::new();
    io::stdin()
        .read_line(&mut username)
        .expect("Не вдалося зчитати логін");
    let username = username.trim();

    // Check username length
    if username.len() < 8 {
        println!("Логін повинен бути мінімум 8 символів!");
        return None;
    }

    // Open users.csv and check if the username already exists in the second column
    let file_path = "src/users.csv";
    if let Ok(file) = File::open(file_path) {
        let reader = BufReader::new(file);
        for line in reader.lines() {
            if let Ok(line) = line {
                let columns: Vec<&str> = line.split(',').collect();
                if columns.len() >= 2 && columns[1].trim() == username {
                    println!("Логін вже існує!");
                    return None;
                }
            }
        }
    }

    // Prompt for a password
    print!("Введіть пароль (мінімум 8 символів): ");
    let _ = io::Write::flush(&mut io::stdout());
    let mut password = String::new();
    io::stdin()
        .read_line(&mut password)
        .expect("Не вдалося зчитати пароль");
    let password = password.trim();

    // Check password length
    if password.len() < 8 {
        println!("Пароль повинен бути мінімум 8 символів!");
        return None;
    }

    // Prompt for password confirmation
    print!("Повторіть пароль: ");
    let _ = io::Write::flush(&mut io::stdout());
    let mut confirm_password = String::new();
    io::stdin()
        .read_line(&mut confirm_password)
        .expect("Не вдалося зчитати повторний пароль");
    let confirm_password = confirm_password.trim();

    // Check if passwords match
    if password != confirm_password {
        println!("Паролі не збігаються!");
        return None;
    }

    // Find the last ID in the file and increment it
    let mut new_id = 1; // Default ID if the file is empty
    if let Ok(file) = File::open(file_path) {
        let reader = BufReader::new(file);
        for line in reader.lines() {
            if let Ok(line) = line {
                let columns: Vec<&str> = line.split(',').collect();
                if let Some(id_str) = columns.get(0) {
                    if let Ok(id) = id_str.trim().parse::<u32>() {
                        new_id = id + 1; // Increment ID
                    }
                }
            }
        }
    }

    // Add the new user to the CSV file
    if let Ok(mut file) = OpenOptions::new()
        .append(true)
        .create(true)
        .open(file_path)
    {
        if let Err(err) = writeln!(file, "{},{},{}", new_id, username, password) {
            println!("Не вдалося записати в файл: {}", err);
            return None;
        }
    } else {
        println!("Не вдалося відкрити файл для запису!");
        return None;
    }

    println!("Реєстрація успішна! Ваш логін: {}", username);
    Some(username.to_string())
}

fn add_task_to_csv(time: String, task: String, username: String) {
    let file_path = "src/tasks.csv";

    let last_id = match File::open(file_path) {
        Ok(file) => {
            let reader = BufReader::new(file);
            // Get the last non-empty line
            reader.lines()
                .filter_map(Result::ok)
                .filter(|line| !line.trim().is_empty())
                .last()
                .and_then(|line| {
                    line.split(',').next()?.parse::<u32>().ok()
                })
                .unwrap_or(0) // Default to 0 if no valid ID found
        }
        Err(_) => 0, // If file doesn't exist, start with ID 0
    };

    let new_id = last_id + 1;

    // Відкриваємо файл у режимі додавання або створюємо новий, якщо його немає
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_path)
        .expect("Failed to open tasks.csv");

    // Write the new row to the file
    let csv_row = format!("{},{},{},{}\n", new_id, time, task, username);
    file.write_all(csv_row.as_bytes())
        .expect("Failed to write to tasks.csv");
}

fn run_todo(username: String) {
    loop {
        let file_path = "src/tasks.csv";

        // Initialize a vector to store tasks as [id, time, task]
        let mut tasks: Vec<Vec<String>> = Vec::new();

        // Open the CSV file
        if let Ok(file) = File::open(file_path) {
            let reader = BufReader::new(file);

            // Read lines from the file
            for line in reader.lines() {
                if let Ok(line) = line {
                    let columns: Vec<&str> = line.split(',').collect();

                    // Ensure the row has at least 4 columns
                    if columns.len() >= 4 {
                        // Check if the 4th column matches the username
                        if columns[3].trim() == username {
                            // Collect the first 3 columns (ID, Time, Task) into a task
                            tasks.push(vec![
                                columns[0].trim().to_string(), // ID
                                columns[1].trim().to_string(), // Time
                                columns[2].trim().to_string(), // Task
                            ]);
                        }
                    }
                }
            }
        } else {
            println!("Не вдалося відкрити tasks.csv!");
            return;
        }

        println!("Ваші завдання:");

        println!("Num    Time          Task");
        for (i, task) in tasks.iter().enumerate() {
            if let [id, time, task_name] = &task[..] {
                println!(
                    "{:<6} {:<8} {}",
                    i + 1,           // Display task number starting from 1
                    time,            // Time column
                    task_name        // Task description
                );
            }
        }

        if tasks.is_empty() {
            println!("Немає завдань для користувача {}", username);
        }

        println!("Виберіть дію:");
        println!("1. Оновити завдання");
        println!("2. Видалити завдання");
        println!("3. Створити завдання");
        println!("4. Вийти з облікового запису");
        print!("Введіть відповідь: ");
        let _ = io::Write::flush(&mut io::stdout());
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Не вдалося зчитати ввід");

        match input.trim().parse::<u32>() {
            Ok(1) => {
                print!("Введіть номер завдання для редагування: ");
                let _ = io::Write::flush(&mut io::stdout());
                let mut ans = String::new();
                io::stdin()
                    .read_line(&mut ans)
                    .expect("Не вдалося зчитати ввід");
                let task_number: usize = ans.trim().parse().unwrap_or(0);

                if task_number == 0 || task_number > tasks.len() {
                    println!("Невірний номер завдання!");
                    return;
                }

                // Знайдемо ID для завдання, яке ми хочемо редагувати
                let task_to_edit = &tasks[task_number - 1]; // Зменшуємо на 1, бо tasks[0] це перше завдання
                let task_id = &task_to_edit[0]; // Перше значення в task це ID

                println!("Що змінити?");
                println!("1. Час");
                println!("2. Завдання");
                print!("Введіть відповідь (1/2): ");
                let _ = io::Write::flush(&mut io::stdout());
                let mut choice = String::new();
                io::stdin()
                    .read_line(&mut choice)
                    .expect("Не вдалося зчитати ввід");
                let choice = choice.trim();

                match choice {
                    "1" => {
                        // Оновлення часу
                        print!("Уведіть новий час: ");
                        let _ = io::Write::flush(&mut io::stdout());
                        let mut new_time = String::new();
                        io::stdin()
                            .read_line(&mut new_time)
                            .expect("Не вдалося зчитати новий час");
                        let new_time = new_time.trim();

                        // Оновлюємо CSV файл
                        let file_path = "src/tasks.csv";
                        let mut updated_lines = Vec::new();

                        if let Ok(file) = File::open(file_path) {
                            let reader = BufReader::new(file);
                            for line in reader.lines() {
                                if let Ok(line) = line {
                                    let columns: Vec<&str> = line.split(',').collect();
                                    if columns.len() >= 4 {
                                        if columns[0].trim() == task_id {
                                            updated_lines.push(format!("{},{},{}", task_id, new_time, columns[2]));
                                        } else {
                                            updated_lines.push(line);
                                        }
                                    }
                                }
                            }
                        } else {
                            println!("Не вдалося відкрити файл tasks.csv!");
                            return;
                        }

                        // Перезаписуємо файл
                        if let Ok(file) = OpenOptions::new().write(true).truncate(true).open(file_path) {
                            let mut writer = BufWriter::new(file);
                            for line in updated_lines {
                                if let Err(e) = writeln!(writer, "{}", line) {
                                    println!("Не вдалося записати в файл: {}", e);
                                    return;
                                }
                            }
                            println!("Час успішно оновлено!");
                        } else {
                            println!("Не вдалося відкрити файл tasks.csv для запису!");
                        }
                    },
                    "2" => {
                        // Оновлення завдання
                        print!("Уведіть новий опис завдання: ");
                        let _ = io::Write::flush(&mut io::stdout());
                        let mut new_task = String::new();
                        io::stdin()
                            .read_line(&mut new_task)
                            .expect("Не вдалося зчитати новий опис завдання");
                        let new_task = new_task.trim();

                        // Оновлюємо CSV файл
                        let file_path = "src/tasks.csv";
                        let mut updated_lines = Vec::new();

                        if let Ok(file) = File::open(file_path) {
                            let reader = BufReader::new(file);
                            for line in reader.lines() {
                                if let Ok(line) = line {
                                    let columns: Vec<&str> = line.split(',').collect();
                                    if columns.len() >= 4 {
                                        if columns[0].trim() == task_id {
                                            updated_lines.push(format!("{},{},{}, {}", task_id, columns[1], new_task, username));
                                        } else {
                                            updated_lines.push(line);
                                        }
                                    }
                                }
                            }
                        } else {
                            println!("Не вдалося відкрити файл tasks.csv!");
                            return;
                        }

                        // Перезаписуємо файл
                        if let Ok(file) = OpenOptions::new().write(true).truncate(true).open(file_path) {
                            let mut writer = BufWriter::new(file);
                            for line in updated_lines {
                                if let Err(e) = writeln!(writer, "{}", line) {
                                    println!("Не вдалося записати в файл: {}", e);
                                    return;
                                }
                            }
                            println!("Опис завдання успішно оновлено!");
                        } else {
                            println!("Не вдалося відкрити файл tasks.csv для запису!");
                        }
                    },
                    _ => {
                        println!("Невірний вибір!");
                    }
                }
            },
            Ok(2) => {
                print!("Введіть номер завдання для видалення: ");
                let _ = io::Write::flush(&mut io::stdout());
                let mut ans = String::new();
                io::stdin()
                    .read_line(&mut ans)
                    .expect("Не вдалося зчитати ввід");
                let task_number: usize = ans.trim().parse().unwrap_or(0);

                if task_number == 0 || task_number > tasks.len() {
                    println!("Невірний номер завдання!");
                    return;
                }

                // Знайдемо ID для завдання, яке ми хочемо видалити
                let task_to_delete = &tasks[task_number - 1]; // Зменшуємо на 1, бо tasks[0] це перше завдання
                let task_id = &task_to_delete[0]; // Перше значення в task це ID

                println!("Ви справді хочете видалити завдання {}?", task_to_delete[2]);  // task_to_delete[2] - це завдання
                print!("Відповідь (Так/Ні): ");
                let _ = io::Write::flush(&mut io::stdout());
                let mut final_ans = String::new();
                io::stdin()
                    .read_line(&mut final_ans)
                    .expect("Не вдалося зчитати ввід");

                if final_ans.trim() == "Так" {
                    // КОД СЮДИ!
                    let file_path = "src/tasks.csv";

                    // Прочитаємо всі рядки з файлу
                    let mut updated_lines = Vec::new();
                    if let Ok(file) = File::open(file_path) {
                        let reader = BufReader::new(file);

                        // Читаємо всі рядки
                        for line in reader.lines() {
                            if let Ok(line) = line {
                                let columns: Vec<&str> = line.split(',').collect();
                                if columns.len() >= 4 {
                                    if columns[0].trim() != task_id {  // Якщо ID не співпадає з тим, що ми хочемо видалити
                                        updated_lines.push(line);  // Додаємо рядок в список для подальшого запису
                                    }
                                }
                            }
                        }
                    } else {
                        println!("Не вдалося відкрити файл tasks.csv для читання!");
                        return;
                    }

                    // Тепер ми перепишемо файл без видаленого рядка
                    if let Ok(file) = OpenOptions::new().write(true).truncate(true).open(file_path) {
                        let mut writer = BufWriter::new(file);

                        // Запишемо всі залишкові рядки назад у файл
                        for line in updated_lines {
                            if let Err(e) = writeln!(writer, "{}", line) {
                                println!("Не вдалося записати в файл: {}", e);
                                return;
                            }
                        }
                        println!("Завдання успішно видалено.");
                    } else {
                        println!("Не вдалося відкрити файл tasks.csv для запису!");
                    }
                } else {
                    println!("Видалення скасовано.");
                }
            },
            Ok(3) => {
                println!("Створення завдання");
                print!("Введіть час: ");
                let _ = io::Write::flush(&mut io::stdout());
                let mut ans = String::new();
                io::stdin()
                    .read_line(&mut ans)
                    .expect("Не вдалося зчитати ввід");
                let new_time = ans.trim();
                print!("Введіть текст завдання: ");
                let _ = io::Write::flush(&mut io::stdout());
                let mut ans2 = String::new();
                io::stdin()
                    .read_line(&mut ans2)
                    .expect("Не вдалося зчитати ввід");
                let new_task = ans2.trim();
                add_task_to_csv(new_time.to_string(), new_task.to_string(), username.to_string());
                println!("Завдання додане!");
            },
            Ok(4) => {
                println!("Вихід з облікового запису...");
                break;
            }
            _ => println!("Невірний вибір. Спробуйте ще раз."),
        }
    }
}

fn main() {
    loop {
        println!("Для використання програми необхідно мати акаунт:");
        println!("1. Увійти в існуючий");
        println!("2. Зареєструватись");
        println!("3. Завершити роботу");
        print!("Введіть відповідь: ");
        let _ = io::Write::flush(&mut io::stdout());
        let mut answer = String::new();
        io::stdin()
            .read_line(&mut answer)
            .expect("Не вдалося зчитати ввід");

        let username: Option<String>; // Initialize the username variable

        match answer.trim().parse::<u32>() {
            Ok(1) => {
                username = login();
            }
            Ok(2) => {
                username = register();
            }
            Ok(3) => {
                println!("Завершення роботи програми...");
                return; // Exit the program
            }
            _ => {
                println!("Невірний вибір. Спробуйте ще раз.");
                username = None; // Explicitly assign None in this case
            }
        }

        match username {
            Some(user) => {
                println!("Welcome, {}!", user);
                // Now you can call run_todo with the username
                run_todo(user);
            }
            None => {
                println!("Login failed or action canceled.");
            }
        }
    }
}