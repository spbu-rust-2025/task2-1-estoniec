use std::io;

fn manaker_palindrom(s: &str) -> String {
    if s.is_empty() {
        return String::new(); // возвращаем пустую сроку в случае пустой строки
    }

    let mut transformed = String::with_capacity(s.len() * 2 + 1);
    transformed.push('#');
    for c in s.chars() {
        transformed.push(c);
        transformed.push('#');
    } // преобразуем изначальную строку в строку вида #[element]#...#[element]#

    let n = transformed.len();
    let mut p = vec![0; n]; // вектор радиусов (расстояние от центра палиндрома
    // до дальнего элемента)
    let mut center = 0; // центр палиндрома
    let mut right = 0; // правая граница самого большого палиндрома
    let mut max_len = 0; // максимальная длина палиндрома
    let mut max_center = 0; // центр максимально большого палиндрома

    for i in 0..n {
        let mirror = 2 * center as i32 - i as i32; // находим зеркальный элемент к текущему,
        // i всегда правее центра

        if i < right {
            p[i] = p[mirror as usize].min(right - i); // вычисляем радиус для текущего элемента
            // таким образом чтобы он был в пределах известного палиндрома
        }

        let mut left_id = i as i32 - (p[i] as i32 + 1); // левая граница текущего палиндрома
        let mut right_id = i + p[i] + 1; // правая граница текущего палиндрома

        while left_id >= 0
            && right_id < n
            && transformed.chars().nth(left_id as usize) == transformed.chars().nth(right_id)
        {
            p[i] += 1;
            left_id -= 1;
            right_id += 1;
        } // просто расширяем радиус пока палиндром

        if i + p[i] > right {
            center = i;
            right = i + p[i];
        } // обновляем значения палиндома

        if p[i] > max_len {
            max_len = p[i];
            max_center = i;
        } // обновляем максимальный палиндром (если нужно, конечно)
    }

    let start = max_center - max_len;
    let end = max_center + max_len;
    transformed
        .chars()
        .skip(start)
        .take(end - start + 1)
        .filter(|&c| c != '#')
        .collect()
}

fn main() -> Result<(), io::Error> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let res = manaker_palindrom(&input);
    println!("{}", res);

    Ok(())
}