use std::io;

fn main() {
    let mut stones = 20; // 石の初期数
    let mut competitor_take = stones - 1; // 対戦相手の石取得数

    println!("石取りゲームを始めます！");

    while stones > 1 {
        println!("現在の石の数: {}", stones);

        // プレイヤーのターン
        let player_take = get_player_input(&stones, &competitor_take);
        stones -= player_take;

        competitor_take = player_take * 2;

        if stones <= 1 {
            println!("プレイヤーの勝利！");
            break;
        }

        // コンピュータのターン
        let computer_take = get_computer_move(&stones, &competitor_take);
        println!("コンピュータが {} 個の石を取りました。", computer_take);
        stones -= computer_take;

        competitor_take = computer_take * 2;

        if stones <= 1 {
            println!("コンピュータの勝利！");
            break;
        }
    }
}

fn get_player_input(stones: &usize, player_limit: &usize) -> usize {
    let max_take = std::cmp::min(*player_limit, *stones);
    loop {
        println!("何個の石を取りますか？(1-{})", max_take);
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
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

fn get_computer_move(stones: &usize, computer_limt: &usize) -> usize {
    // 相手が直前に取った石の数の2倍を超えないように、最大で残りの石の半分まで取る。
    let max_take = std::cmp::min(*computer_limt, *stones);
    if max_take == 0 {
        1
    } else {
        max_take
    }
}
