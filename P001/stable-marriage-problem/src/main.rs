// 下記ブログの安定結婚問題を解く
// 安定マッチングとGale-Shapleyアルゴリズム | 高校数学の美しい物語
// https://manabitimes.jp/math/1078
fn main() {
    // 男性の好み
    let boy_preference = [[1_usize, 2, 0], [2, 1, 0], [0, 2, 1]];
    // 女性の好み
    let girl_preference = [[2_usize, 1, 0], [1, 2, 0], [2, 1, 0]];

    // 男性の結婚状況
    let mut boy_pair = [-1_i16; 3_usize];
    // 女性の結婚状況
    let mut girl_pair = [-1_i16; 3_usize];

    // 全てのペアができるまで繰り返す
    while boy_pair.iter().any(|&x| x < 0_i16) {
        for i_b in 0..boy_pair.len() {
            // 既に相手がいる場合
            if boy_pair[i_b] >= 0_i16 {
                continue;
            }

            println!("\n{}番目の男性の好み: {:?}", (i_b + 1), boy_preference[i_b]);
            let mut order = [0_usize; 5_usize];
            for (index, item) in boy_preference[i_b].iter().enumerate() {
                order[*item] = index;
            }
            //println!("プロポーズの順番: {:?}", order);

            // プロポーズ
            for i_o in 0..order.len() {
                let i_g = order[i_o];
                // 女性に相手がいない場合
                if girl_pair[i_g] < 0_i16 {
                    boy_pair[i_b] = i_g as i16;
                    girl_pair[i_g] = i_b as i16;
                    break;
                } else {
                    // プロポーズ受諾済みの場合
                    let current_b = girl_pair[i_g] as usize;
                    // 今回の男性の方が好みだった場合
                    if girl_preference[i_g][current_b] > girl_preference[i_g][i_b] {
                        boy_pair[i_b] = i_g as i16;
                        girl_pair[i_g] = i_b as i16;

                        boy_pair[current_b] = -1_i16;
                        break;
                    }
                }
            }
        }
    }

    println!("\n結果");
    for i_g in 0..girl_pair.len() {
        println!("{} - {}", i_g, girl_pair[i_g]);
    }
}