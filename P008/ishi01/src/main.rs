use std::io;

fn main() {
    let mut stones = 20; // 石の初期数
    let max_take = 3; // 1回に取れる最大数

    println!("石取りゲームを始めます！");
    println!("最大 {} 個までの石を取ることができます。", max_take);

    while stones > 0 {
        println!("現在の石の数: {}", stones);

        // プレイヤーのターン
        let player_take = get_player_input(max_take);
        stones -= player_take;

        if stones <= 0 {
            println!("プレイヤーの勝利！");
            break;
        }

        // コンピュータのターン
        let computer_take = get_computer_move(stones, max_take);
        println!("コンピュータが {} 個の石を取りました。", computer_take);
        stones -= computer_take;

        if stones <= 0 {
            println!("コンピュータの勝利！");
            break;
        }
    }
}

fn get_player_input(max_take: usize) -> usize {
    loop {
        println!("何個の石を取りますか？(1-{}個)", max_take);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input: usize = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("不正な入力です。数字を入力してください。");
                continue;
            }
        };
        if input > 0 && input <= max_take {
            return input;
        } else {
            println!("1から{}までの数字を入力してください。", max_take);
        }
    }
}

fn get_computer_move(stones: usize, max_take: usize) -> usize {
    // 簡単な戦略として、残りの石が1つ以上max_takeより多い場合は、max_take個の石を取る。
    // そうでない場合は、残りの石の数を取る。
    if stones > max_take {
        max_take
    } else {
        stones
    }
}
