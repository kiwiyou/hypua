include!(concat!(env!("OUT_DIR"), "/codegen.rs"));

/// PUA 코드를 담은 `char` 값을 첫가끝 방식의 문자열로 변환합니다.
/// 변환에 성공한 경우 `Some(&'static str)`, 실패한 경우 `None`을 반환합니다.
/// # 예제
/// ```
/// use hypua::to_ipf;
/// 
/// // U+E4CF () = ᄙᅰ
/// assert_eq!(to_ipf(''), Some("ᄙᅰ"));
/// // 한양 PUA 코드가 아니라면 `None`을 반환합니다.
/// assert_eq!(to_ipf('사'), None);
/// ```
pub fn to_ipf(pua: char) -> Option<&'static str> {
    TABLE.get(&pua).cloned()
}
