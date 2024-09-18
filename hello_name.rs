// Определение трейта Action с методом say
trait Action {
    fn say(&self);
}

// Определение структуры Person
struct Person {
    name: String,
}

// Реализация трейта Action для структуры Person
impl Action for Person {
    fn say (&self) {
        // Преобразование имени в верхний регистр
        let upper_name = self.name.to_uppercase();
        println!("Hello, {}", upper_name);
    }
}

fn main() {
    // Создание экземпляра Person
    let person = Person{
        name: String::from("Bogdan"),
    };

    // Вызов метода say
    person.say();
}
