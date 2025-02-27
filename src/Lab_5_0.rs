/*

mod Lab_5_0;

#[derive(Debug)]
/// Событие лифта на которое должен реагировать контроллер.
enum Event {
    // TODO:добавьте необходимые варианты
    // Перечисление возможных событий
    CurrentFloor(i32), // Текущий этаж
    DoorOpened, // Двери лифта открыты
    DoorClosed, // Двери лифта закрыты
    CallFromFloor(i32, Direction), // Вызов с определённого этажа вверх/вниз
    ChosenFloor(i32), // Выбранный пассажиром этаж
    Stop, // Лифт остановился
    Crash, // Лифт сломался
    Maintenance, // Лифт на ремонте
}

/// A direction of travel.
/// Направление движения лифта
#[derive(Debug)]
enum Direction {
    Up,
    Down,
}

/// Кабина приехала на заданный этаж.
fn car_arrived(floor: i32) -> Event {
    Event::CurrentFloor(floor)
}

/// Двери кабины открыты.
fn car_door_opened() -> Event {
    Event::DoorOpened
}

/// Двери кабины закрыты.
fn car_door_closed() -> Event {
    Event::DoorClosed
}

/// Кнопка вызова лифта нажата на заданном этаже.
fn lobby_call_button_pressed(floor: i32, dir: Direction) -> Event {
    Event::CallFromFloor(floor, dir)
}

/// Кнопка этажа нажата в кабине лифта.
fn car_floor_button_pressed(floor: i32) -> Event {
    Event::ChosenFloor(floor)
}

fn main() {
    println!(
        "Пассажир на первом этаже нажал кнопку вызова: {:?}",
        //lobby_call_button_pressed(0, Direction::Up)
        lobby_call_button_pressed(1, Direction::Up)
    );
    //println!("Лифт приехал на первый этаж: {:?}", car_arrived(0));
    println!("Лифт приехал на первый этаж: {:?}", car_arrived(1));
    println!("Дверь лифта открылась: {:?}", car_door_opened());
    println!(
        "Пассажир нажал кнопку третьего этажа: {:?}",
        car_floor_button_pressed(3)
    );
    println!("Двери лифта закрылись: {:?}", car_door_closed());
    println!("Лифт прибыл на третий этаж: {:?}", car_arrived(3));
}

//fn main() {
//    println!("Hello, world!");
//}
*/