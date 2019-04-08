/// デフォルトのパスワード長さ
pub const DEFAULT_PASS_LEN: i32 = 16;
/// パスワードに使う文字列（フォントによって見分けにくいものを除外）
pub static PASS_PHRASE: &str = "abcdefghijkmnprstuvwxyABCDEFGHJKLMNPQRSTUVWXYZ2345678";
/// パスワードに使用可能な記号
pub const SPECIAL_CHARS: [char; 16] = [
    '!', '#', '$', '%', '&', '*', ']', '[', '(', ')', '{', '}', '+', '-', '_', '.'
];

