/* DEBUG
// TODO: закомментируйте эту строчку, когда закончите отладку программы.
#![allow(unused_variables, dead_code)]


#![allow(dead_code)]
pub struct User {
    name: String,
    age: u32,
    height: f32,
    visit_count: u32,
    last_blood_pressure: Option<(u32, u32)>,
}

pub struct Measurements {
    height: f32,
    blood_pressure: (u32, u32),
}

pub struct HealthReport<'a> {
    patient_name: &'a str,
    visit_count: u32,
    height_change: f32,
    blood_pressure_change: Option<(i32, i32)>,
}

impl User {
    pub fn new(name: String, age: u32, height: f32) -> Self {
        Self { name, age, height, visit_count: 0, last_blood_pressure: None }
    }

    pub fn visit_doctor(&mut self, measurements: Measurements) -> HealthReport {
        //todo!("Обновление данных после визита к врачу")
        // Обновление количеств визитов к врачу
        self.visit_count += 1;
        //self.height = measurements.height;
        //self.last_blood_pressure = measurements.blood_pressure.into();
        // Вычисление разницы давлений
        let blood_pressure_change_value = self.last_blood_pressure.map(|(p1, p2)| {
            //let (p1 as i32, p2 as i32) = self.last_blood_pressure;
            let (n1, n2) = measurements.blood_pressure;
            //(p1 as i32 - n1 as i32, p2 as i32 - n2 as i32)
            (n1 as i32 - p1 as i32, n2 as i32 - p2 as i32)
        });
        // Сохранение текущего роста
        let patient_height_before = self.height;
        // Обновление данных роста и давления
        self.height = measurements.height;
        //self.last_blood_pressure = Option::from(measurements.blood_pressure);
        self.last_blood_pressure = Some(measurements.blood_pressure);
        // Составление отчёта
        HealthReport {
            patient_name: self.name.as_str(),
            visit_count: self.visit_count,
            height_change: measurements.height - patient_height_before,
            //blood_pressure_change: Option::from(self.last_blood_pressure - measurements.blood_pressure),
            //blood_pressure_change: self.last_blood_pressure - measurements.blood_pressure,
            blood_pressure_change: blood_pressure_change_value,
        }
    }
}

fn main() {
    let bob = User::new(String::from("Bob"), 32, 155.2);
    println!("Меня зовут {} и мой возраст {}", bob.name, bob.age);
    // Дополнение
    let mut jim = User::new(String::from("Джим"), 20, 190.1);
    jim.visit_doctor(Measurements{height: 155.2, blood_pressure: (121, 80),});
    println!("Данные по Джиму Хокинсу == \n name : {},\n age: {},\n height: {},\n visit_count: {},\n blood_pressure: {:?}",
             jim.name, jim.age, jim.height, jim.visit_count, jim.last_blood_pressure);
    println!(" Характер спокойный.\n Не женат.");
}

#[test]
fn test_visit() {
    let mut bob = User::new(String::from("Гиппократ"), 32, 155.2);
    assert_eq!(bob.visit_count, 0);
    let report =
        bob.visit_doctor(Measurements { height: 156.1, blood_pressure: (120, 80) });
    assert_eq!(report.patient_name, "Гиппократ");
    assert_eq!(report.visit_count, 1);
    assert_eq!(report.blood_pressure_change, None);
    assert!((report.height_change - 0.9).abs() < 0.00001);

    let report =
        bob.visit_doctor(Measurements { height: 156.1, blood_pressure: (115, 76) });

    assert_eq!(report.visit_count, 2);
    assert_eq!(report.blood_pressure_change, Some((-5, -4)));
    assert_eq!(report.height_change, 0.0);
}







//fn main() {
//    println!("Hello, world!");
//}
*/