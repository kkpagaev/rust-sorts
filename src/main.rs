use std::env;
use std::thread;
use std::time::Instant;

fn heapify(arr: &mut [&str], n: usize, i: usize) {
    let mut largest = i;
    let left = 2 * i + 1;
    let right = 2 * i + 2;

    if left < n && arr[left] >= arr[largest] {
        largest = left;
    }

    if right < n && arr[right] > arr[largest] {
        largest = right;
    }

    if largest != i {
        arr.swap(i, largest);
        heapify(arr, n, largest);
    }
}

fn heap_sort(arr: &mut [&str]) {
    let n = arr.len();

    for i in (0..n / 2).rev() {
        heapify(arr, n, i);
    }

    for i in (0..n).rev() {
        arr.swap(0, i);
        heapify(arr, i, 0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut ar = vec!["Z", "A", "E", "F", "O"];
        heap_sort(&mut ar);
        assert_eq!(ar, vec!["A", "E", "F", "O", "Z"]);
    }
}

fn quick_sort(arr: &mut [&str], low: isize, high: isize) {
    if low < high {
        let p = partition(arr, low, high);
        quick_sort(arr, low, p);
        quick_sort(arr, p + 1, high);
    }
}

fn partition(arr: &mut [&str], low: isize, high: isize) -> isize {
    let pivot = arr[low as usize];

    let mut i = low - 1;
    let mut j = high + 1;

    loop {
        i += 1;
        while arr[i as usize] < pivot {
            i += 1;
        }

        j -= 1;
        while arr[j as usize] > pivot {
            j -= 1;
        }

        if i >= j {
            return j;
        }

        arr.swap(i as usize, j as usize);
    }
}

fn parallel_quick_sort(arr: &mut [&str], threads: usize) {
    let chunks = std::cmp::min(arr.len(), threads);
    let mut chunk_size = if arr.len() / chunks % 2 == 0 {
        arr.len() / chunks
    } else {
        arr.len() / chunks + 1
    };

    thread::scope(|scope| {
        for slice in arr.chunks_mut(chunk_size) {
            scope.spawn(|| quick_sort(slice, 0, (slice.len() - 1) as isize));
        }
    });
    let n = arr.len();

    while chunk_size < n {
        chunk_size *= 2;
        thread::scope(|scope| {
            for slice in arr.chunks_mut(chunk_size) {
                scope.spawn(|| {
                    let left = 0;
                    let right = slice.len();
                    let mid = chunk_size / 2 % right;
                    merge(slice, left, mid, right)
                });
            }
        });
    }
}

fn parallel_heap_sort(arr: &mut [&str], threads: usize) {
    let chunks = std::cmp::min(arr.len(), threads);
    let mut chunk_size = if arr.len() / chunks % 2 == 0 {
        arr.len() / chunks
    } else {
        arr.len() / chunks + 1
    };
    // println!("org: {}, chunks: {}, chunk_size: {}", org, chunks, chunk_size);

    thread::scope(|scope| {
        for slice in arr.chunks_mut(chunk_size) {
            scope.spawn(|| heap_sort(slice));
        }
    });
    let n = arr.len();

    while chunk_size < n {
        chunk_size *= 2;
        thread::scope(|scope| {
            for slice in arr.chunks_mut(chunk_size) {
                scope.spawn(|| {
                    let left = 0;
                    let right = slice.len();
                    let mid = chunk_size / 2 % right;
                    merge(slice, left, mid, right)
                });
            }
        });
    }
}

fn merge(arr: &mut [&str], left: usize, mid: usize, right: usize) {
    let left_arr = arr[left..mid].to_vec();
    let right_arr = arr[mid..right].to_vec();

    let mut i = 0;
    let mut j = 0;
    let mut k = left;

    while i < left_arr.len() && j < right_arr.len() {
        if left_arr[i] < right_arr[j] {
            arr[k] = left_arr[i];
            i += 1;
        } else {
            arr[k] = right_arr[j];
            j += 1;
        }
        k += 1;
    }

    while i < left_arr.len() {
        arr[k] = left_arr[i];
        i += 1;
        k += 1;
    }

    while j < right_arr.len() {
        arr[k] = right_arr[j];
        j += 1;
        k += 1;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let n = &args[1].parse::<u32>().unwrap();
    let is_print = args[3] == "true";
    let mut data: Vec<&str> = vec![];
    for i in 0..*n {
        data.push(NAMES[(i % 200) as usize]);
    }

    let num_threads = args[2].parse::<usize>().unwrap();

    let start_time = Instant::now();
    if args.get(4).is_some() {
        parallel_quick_sort(&mut data, num_threads);
    } else {
        parallel_heap_sort(&mut data, num_threads);
    }
    let end_time = Instant::now();

    if is_print {
        for d in data {
            println!("{}", d);
        }
    } else {
        println!("{}", (end_time - start_time).as_secs_f64());
    }
}

static NAMES: &'static [&'static str] = &[
    // "G",
    // "C",
    // "W",
    // "U",
    // "V",
    // "Y",
    // "L",
    // "S",
    // "T",
    // "H",
    // "K",
    // "B",
    // "X",
    // "J",
    // "P",
    // "R",
    // "D",
    // "M",
    // "Z",
    // "N",
    // "I",
    // "O",
    // "A",
    // "E",
    // "Q",
    // "F",
    "Мороховец Микола Олегович 1961.02.02",
    "Сидоров Сергій Миколаївич 1990.11.11",
    "Погребинський Олександр Сергійович 1945.09.21",
    "Бондарев Володимир Володимирович 1960.08.25",
    "Федюрко Євген Іванович 1962.05.23",
    "Ковальчук Марія Іванівна 1975.12.14",
    "Петрова Олена Миколаївна 1978.04.03",
    "Савченко Олексій Вікторович 1992.09.15",
    "Гончаренко Тетяна Сергіївна 1980.11.28",
    "Коваленко Василь Петрович 1967.07.17",
    "Семенов Андрій Олександрович 1989.02.19",
    "Мельник Людмила Павлівна 1972.03.30",
    "Лисенко Олександра Іванівна 1983.05.12",
    "Григоренко Віталій Віталійович 1975.12.01",
    "Шевченко Марина Степанівна 1988.07.24",
    "Козаченко Ірина Петрівна 1970.09.03",
    "Чернов Михайло Олегович 1995.01.08",
    "Павленко Олег Васильович 1969.10.26",
    "Ткаченко Анна Григорівна 1987.08.18",
    "Захарченко Віктор Михайлович 1977.06.14",
    "Гончарук Оксана Олександрівна 1981.04.07",
    "Корчак Ігор Ігорович 1991.03.22",
    "Максимова Лілія Юріївна 1984.12.05",
    "Литвиненко Дмитро Юрійович 1979.02.23",
    "Соколова Ірина Михайлівна 1973.11.16",
    "Романенко Анатолій Валентинович 1965.09.09",
    "Левченко Світлана Петрівна 1986.01.12",
    "Кудрявцев Сергій Олексійович 1993.08.31",
    "Василенко Ольга Сергіївна 1971.06.28",
    "Панченко Вікторія Василівна 1989.04.04",
    "Ковальов Олексій Михайлович 1982.10.11",
    "Зінченко Ірина Володимирівна 1974.07.20",
    "Поляков Владислав Павлович 1994.05.27",
    "Шевчук Наталія Олегівна 1976.03.15",
    "Лисенко Віктор Олександрович 1968.12.08",
    "Жукова Ірина Василівна 1980.08.22",
    "Кузьменко Максим Андрійович 1990.01.03",
    "Головченко Ольга Вікторівна 1978.09.29",
    "Савчук Олександр Петрович 1966.07.13",
    "Попова Лариса Геннадіївна 1983.06.16",
    "Шаповал Віталій Олегович 1972.04.02",
    "Коваль Оксана Ігорівна 1992.02.20",
    "Даниленко Валентин Миколайович 1971.11.11",
    "Петрик Ірина Василівна 1987.10.14",
    "Литвин Олег Сергійович 1964.08.27",
    "Гриненко Тетяна Олександрівна 1976.04.01",
    "Коваленко Володимир Іванович 1993.03.18",
    "Жуков Вадим Васильович 1985.02.06",
    "Сидоренко Ольга Віталіївна 1979.07.23",
    "Павленко Денис Михайлович 1969.05.29",
    "Козачук Юлія Вікторівна 1981.12.07",
    "Мельник Віктор Миколайович 1990.11.25",
    "Любченко Наталія Петрівна 1974.09.03",
    "Гавриленко Максим Вікторович 1967.06.12",
    "Семенюк Лариса Сергіївна 1983.01.17",
    "Пономаренко Олександр Володимирович 1970.08.09",
    "Черненко Олена Валентинівна 1992.04.22",
    "Ковальчук Олексій Вікторович 1975.03.10",
    "Гриценко Марія Олегівна 1988.07.28",
    "Тимченко Андрій Михайлович 1977.12.02",
    "Шевченко Наталія Ігорівна 1966.10.15",
    "Ковальчук Дмитро Сергійович 1991.09.05",
    "Попов Віктор Олександрович 1972.02.28",
    "Зіновій Людмила Миколаївна 1986.06.20",
    "Савінов Сергій Вікторович 1978.08.24",
    "Морозова Ірина Олегівна 1994.05.14",
    "Бондаренко Олег Валентинович 1968.03.07",
    "Головко Вікторія Василівна 1980.01.31",
    "Кузьменко Марина Максимівна 1973.11.26",
    "Сергієнко Ігор Володимирович 1984.09.19",
    "Максимов Іван Михайлович 1965.07.21",
    "Поліщук Юлія Андріївна 1982.04.13",
    "Шаповалов Валентин Петрович 1971.02.10",
    "Коваленко Олена Олександрівна 1995.06.26",
    "Лисиченко Андрій Сергійович 1977.12.30",
    "Павлова Ганна Ігорівна 1989.03.04",
    "Гончар Михайло Олегович 1970.05.08",
    "Зінченко Оксана Віталіївна 1983.08.11",
    "Ковальчук Валерій Володимирович 1969.04.12",
    "Жуковська Марія Андріївна 1981.11.07",
    "Петришин Віталій Сергійович 1973.07.03",
    "Литвиненко Наталія Петрівна 1992.02.14",
    "Григорчук Андрій Олександрович 1976.05.26",
    "Сидорова Оксана Миколаївна 1984.08.29",
    "Кузьменко Валентин Михайлович 1967.03.19",
    "Морозов Олег Васильович 1988.09.08",
    "Шевченко Юлія Віталіївна 1979.01.30",
    "Павлов Олександр Валентинович 1971.10.24",
    "Бондаренко Оксана Вікторівна 1990.12.13",
    "Гончаров Михайло Олександрович 1982.06.03",
    "Лисиченко Анна Михайлівна 1964.08.18",
    "Коваленко Володимир Васильович 1974.04.01",
    "Литвиненко Ірина Олегівна 1987.07.12",
    "Савченко Василь Вікторович 1975.02.09",
    "Петров Олег Михайлович 1966.11.28",
    "Гончаренко Тетяна Валентинівна 1989.05.05",
    "Коваль Ігор Олександрович 1977.08.21",
    "Зіновій Наталія Василівна 1993.03.22",
    "Максимов Олександр Володимирович 1980.12.10",
    "Шаповалова Ольга Миколаївна 1968.02.16",
    "Пономаренко Валентин Сергійович 1994.07.29",
    "Гриценко Тамара Іванівна 1972.09.27",
    "Ковальчук Вікторія Олегівна 1986.06.08",
    "Жуковський Валерій Андрійович 1978.04.17",
    "Лисенко Анна Володимирівна 1965.01.05",
    "Петрик Сергій Олександрович 1985.03.15",
    "Мороз Іван Віталійович 1976.08.20",
    "Сидоренко Марія Олегівна 1991.11.25",
    "Гончарук Валентин Вікторович 1983.07.07",
    "Кузьменко Ірина Михайлівна 1969.10.09",
    "Шевчук Олександр Максимович 1980.09.02",
    "Коваленко Вікторія Андріївна 1974.06.06",
    "Павлова Лариса Сергіївна 1987.05.24",
    "Петришин Олег Олександрович 1992.01.28",
    "Гриненко Юлія Володимирівна 1978.04.11",
    "Коваль Оксана Василівна 1985.06.16",
    "Лисиченко Олег Вікторович 1970.09.03",
    "Жуков Володимир Сергійович 1994.03.22",
    "Петрова Валентина Олегівна 1981.11.19",
    "Гончаренко Михайло Миколайович 1975.08.02",
    "Ковал Ольга Петрівна 1990.07.14",
    "Максимов Сергій Валерійович 1973.12.30",
    "Сидоренко Вікторія Ігорівна 1967.02.17",
    "Пономаренко Олексій Олександрович 1984.10.07",
    "Литвин Андрій Сергійович 1979.05.26",
    "Шаповал Вікторія Володимирівна 1993.09.15",
    "Кузьменко Михайло Максимович 1987.08.04",
    "Гончарук Оксана Олександрівна 1966.06.11",
    "Павлов Сергій Ігорович 1971.04.03",
    "Морозова Марія Віталіївна 1982.03.28",
    "Коваль Олексій Володимирович 1968.01.23",
    "Зіновій Анна Ігорівна 1995.05.31",
    "Савченко Василь Васильович 1988.07.12",
    "Григорчук Юрій Васильович 1972.11.09",
    "Лисенко Оксана Володимирівна 1977.09.14",
    "Петрик Олександр Миколайович 1989.04.25",
    "Коваленко Ірина Олегівна 1976.12.03",
    "Гончаров Андрій Сергійович 1983.02.15",
    "Шевчук Анна Віталіївна 1969.08.31",
    "Сидоров Ігор Олександрович 1974.07.27",
    "Пономаренко Ольга Олегівна 1991.10.12",
    "Ковальчук Володимир Олександрович 1980.06.23",
    "Жукова Тетяна Василівна 1965.03.05",
    "Максимов Валентин Вікторович 1979.01.27",
    "Гриненко Анна Сергіївна 1986.05.18",
    "Петрова Лілія Олегівна 1970.09.09",
    "Ковал Олег Валерійович 1994.08.01",
    "Литвиненко Вікторія Іванівна 1978.06.29",
    "Гончарук Олег Максимович 1966.04.04",
    "Шаповал Василь Михайлович 1982.02.20",
    "Кузьменко Ірина Володимирівна 1993.11.07",
    "Савченко Денис Андрійович 1977.10.15",
    "Павлов Віктор Вікторович 1989.12.28",
    "Мороз Олексій Михайлович 1974.08.05",
    "Лисиченко Олена Олександрівна 1980.07.22",
    "Коваленко Андрій Олександрович 1969.06.24",
    "Григорчук Людмила Вікторівна 1986.04.13",
    "Зіновій Ігор Петрович 1972.03.17",
    "Сидорова Марина Олександрівна 1981.01.16",
    "Петришин Андрій Вікторович 1976.12.10",
    "Гриненко Андрій Олегович 1992.05.03",
    "Коваль Ірина Андріївна 1968.07.29",
    "Литвин Володимир Васильович 1985.08.11",
    "Жуков Валентина Сергіївна 1970.06.14",
    "Петров Віктор Михайлович 1994.03.24",
    "Гончаренко Вікторія Андріївна 1988.10.19",
    "Ковал Ольга Миколаївна 1972.11.08",
    "Максимов Володимир Максимович 1966.09.27",
    "Савченко Марія Олегівна 1979.12.05",
    "Пономаренко Ольга Андріївна 1983.02.18",
    "Литвиненко Андрій Петрович 1990.07.07",
    "Шаповал Валерій Олександрович 1984.04.29",
    "Кузьменко Віктор Володимирович 1967.03.12",
    "Гончарук Ольга Сергіївна 1971.08.22",
    "Павлов Вікторія Петрівна 1993.05.26",
    "Морозов Михайло Олегович 1987.09.14",
    "Коваленко Олександр Максимович 1975.01.31",
    "Жуковська Вікторія Ігорівна 1989.08.07",
    "Сидоренко Олег Олександрович 1976.06.30",
    "Петров Сергій Валентинович 1991.03.27",
    "Гриненко Тетяна Вікторівна 1982.12.04",
    "Лисиченко Олег Михайлович 1969.11.25",
    "Ковальчук Лариса Володимирівна 1974.10.13",
    "Максимов Андрій Андрійович 1980.04.06",
    "Шаповалов Олександр Вікторович 1977.07.02",
    "Кузьменко Ірина Сергіївна 1965.02.21",
    "Пономаренко Віктор Сергійович 1986.08.15",
    "Гончаренко Людмила Миколаївна 1995.06.09",
    "Савчук Сергій Олександрович 1973.04.18",
    "Литвиненко Ольга Михайлівна 1978.03.26",
    "Петров Максим Миколайович 1987.02.12",
    "Ковал Олег Віталійович 1964.09.30",
    "Жукова Валентина Володимирівна 1994.08.23",
    "Гриненко Ігор Сергійович 1989.07.31",
    "Кузьменко Володимир Васильович 1971.06.20",
    "Савченко Анна Олегівна 1984.05.17",
    "Петров Валерій Максимович 1970.04.09",
    "Коваленко Тетяна Олегівна 1983.11.07",
    "Лисиченко Олена Віталіївна 1968.10.01",
    "Пономаренко Ольга Василівна 1992.09.11",
    "Морозов Віктор Олегович 1975.08.28",
    "Ковальчук Сергій Володимирович 1981.07.25",
    "Гончарук Ірина Петрівна 1967.06.13",
    "Сидоренко Марія Василівна 1986.02.03",
    "Петришин Оксана Миколаївна 1979.12.22",
    "Жуков Ігор Валерійович 1993.03.14",
    "Кузьменко Ольга Сергіївна 1974.01.08",
    "Гриненко Вікторія Вікторівна 1985.06.06",
    "Максимов Валентин Валентинович 1978.04.20",
    "Савчук Ігор Андрійович 1991.10.02",
    "Коваленко Лариса Василівна 1966.09.19",
    "Пономаренко Ігор Васильович 1982.07.01",
    "Шаповалов Василь Сергійович 1977.05.11",
    "Гончаренко Марія Михайлівна 1994.03.30",
    "Лисиченко Вікторія Олегівна 1971.02.16",
    "Сидоренко Олег Володимирович 1989.01.05",
];
