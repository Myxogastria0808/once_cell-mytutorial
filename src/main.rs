use once_cell::sync::OnceCell;

fn main() {
    //static変数の宣言と初期化
    static INSTANCE: OnceCell<&str> = OnceCell::new();
    //具体的なobject(値)を設定する
    let _ = INSTANCE.set("THIS IS A STATIC INSTANCE");
    // &'static Object を取得
    let instance = INSTANCE.get().unwrap();
    println!("INSCTANCE: {}", instance);
}
