//! PUA 코드를 첫가끝 (IPF) 방식으로 변환하는 라이브러리입니다.
//!
//! # 사용 예
//!
//! ```
//! use hypua::to_ipf_string;
//! 
//! let text = "이런 젼로 어린 百姓이 니르고져  배 이셔도.";
//! println!("{}", to_ipf_string(text));
//! ```

include!(concat!(env!("OUT_DIR"), "/codegen.rs"));

/// PUA 코드를 담은 `char` 값을 첫가끝 방식의 문자열로 변환합니다.
///
/// 변환에 성공한 경우 `Some(&'static str)`, 실패한 경우 `None`을 반환합니다.
///
/// # 사용 예
///
/// ```
/// use hypua::to_ipf;
///
/// // U+E4CF () = ᄙᅰ
/// assert_eq!(to_ipf(''), Some("ᄙᅰ"));
///
/// // 한양 PUA 코드가 아니라면 `None`을 반환합니다.
/// assert_eq!(to_ipf('사'), None);
/// ```
pub fn to_ipf(pua: char) -> Option<&'static str> {
    TABLE.get(&pua).cloned()
}

/// 특정 문자가 PUA 문자인지 아닌지 확인합니다.
///
/// PUA 영역에 포함되면 `true`, 아니라면 `false`를 반환합니다.
pub fn is_pua(pua: char) -> bool {
    (0xE0BC..=0xF8F7).contains(&(pua as u32))
}

/// 문자열에 포함된 PUA 문자를 해당되는 IPF 문자열로 변환합니다.
///
/// # 인수
///
/// * `pua_str` - IPF로 변환할 문자열입니다.
///
/// # 사용 예
///
/// ```
/// use hypua::to_ipf_string;
///
/// // 문장 출처: 분류두공부시언해 초간본 7:2
/// assert_eq!(
///     to_ipf_string(" 雙ㅅ 믌 相對야 락 락 다.").as_ref(),
///     "ᄒᆞᆫ 雙ㅅ 믌ᄃᆞᆯᄀᆞᆫ 相對ᄒᆞ야 ᄌᆞᄆᆞ락 ᄠᅳ락 ᄒᆞᄂᆞ다."
/// );
///
/// assert_eq!(
///     to_ipf_string("이 문장은 PUA 문자를 포함하지 않습니다.").as_ref(),
///     "이 문장은 PUA 문자를 포함하지 않습니다."
/// );
/// ```
pub fn to_ipf_string<'a>(pua_str: &'a str) -> std::borrow::Cow<'a, str> {
    let pua_index = pua_str.find(is_pua);
    if let Some(index) = pua_index {
        let (non_pua, mut pua_str) = pua_str.split_at(index);
        let mut buffer = non_pua.to_owned();
        loop {
            let non_pua_index = pua_str.find(|letter| !is_pua(letter));
            match non_pua_index {
                Some(non_pua_index) => {
                    let (pua, non_pua) = pua_str.split_at(non_pua_index);
                    pua.chars()
                        .flat_map(to_ipf)
                        .for_each(|ipf| buffer.push_str(ipf));
                    pua_str = non_pua;
                }
                None => {
                    pua_str
                        .chars()
                        .flat_map(to_ipf)
                        .for_each(|ipf| buffer.push_str(ipf));
                    break std::borrow::Cow::Owned(buffer);
                }
            }
            let pua_index = pua_str.find(is_pua);
            match pua_index {
                Some(pua_index) => {
                    let (non_pua, pua) = pua_str.split_at(pua_index);
                    buffer.push_str(non_pua);
                    pua_str = pua;
                }
                None => {
                    buffer.push_str(pua_str);
                    break std::borrow::Cow::Owned(buffer);
                }
            }
        }
    } else {
        std::borrow::Cow::Borrowed(pua_str)
    }
}

/// [`std::str::Chars`](https://doc.rust-lang.org/std/str/struct.Chars.html)와 같으나, PUA 문자가 있으면 IPF로 변환되어 한 글자씩 반환하는 `Iterator`입니다.
///
/// [`IntoIpfIterator::into_ipf_iter`](trait.IntoIpfIterator.html#tymethod.ipf_iter)를 통해 얻을 수 있습니다.
pub struct IpfIterator<'a> {
    chars: std::str::Chars<'a>,
    buffer: std::str::Chars<'static>,
}

impl<'a> Iterator for IpfIterator<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next_ipf) = self.buffer.next() {
            Some(next_ipf)
        } else {
            if let Some(new_char) = self.chars.next() {
                if let Some(ipf) = to_ipf(new_char) {
                    self.buffer = ipf.chars();
                    self.next()
                } else {
                    Some(new_char)
                }
            } else {
                None
            }
        }
    }
}

/// [`ipf_iter`](trait.IntoIpfIterator.html#tymethod.ipf_iter)을 구현하는 `trait`입니다.
///
/// `str` 타입에 대해 기본적으로 구현되어 있습니다.
pub trait IntoIpfIterator<'a> {
    /// [`std::str::Chars`](https://doc.rust-lang.org/std/str/struct.Chars.html) 와 같으나, PUA 문자가 있으면 IPF로 변환하는 `Iterator`을 반환합니다.
    ///
    /// IPF로 변환할 때, 반드시 초성 - 중성 - 종성 순으로 반환함이 보장됩니다.
    ///
    /// # 사용 예
    ///
    /// ```
    /// use hypua::IntoIpfIterator;
    ///
    /// let text = "믌";
    /// let char_vec: Vec<char> = text.ipf_iter().collect();
    /// assert_eq!(char_vec, vec!['믌', 'ᄃ', 'ᆞ', 'ᆰ']);
    /// ```
    fn ipf_iter(&'a self) -> IpfIterator<'a>;
}

impl IntoIpfIterator<'_> for str {
    fn ipf_iter(&self) -> IpfIterator {
        IpfIterator {
            chars: self.chars(),
            buffer: "".chars(),
        }
    }
}
