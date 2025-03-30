// errors1.rs
//
//如果通过
//它是一个空字符串。如果解释问题是什么，那会更好，
//而不是有时返回“无”。值得庆幸的是，Rust有类似的
//构造可用于表达错误条件的“结果”。让我们使用
// 它！
//
//执行`rustlings提示错误1`或使用`
// 暗示。


pub fn generate_nametag_text(name: String) ->Result<String, String> {
    if name.is_empty() {
        // Empty names aren't allowed.
        // None
        Err("`name` was empty; it must be nonempty.".into())
    } else {
        Ok(format!("Hi! My name is {}", name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_nametag_text_for_a_nonempty_name() {
        assert_eq!(
            generate_nametag_text("Beyoncé".into()),
            Ok("Hi! My name is Beyoncé".into())
        );
    }

    #[test]
    fn explains_why_generating_nametag_text_fails() {
        assert_eq!(
            generate_nametag_text("".into()),
            // Don't change this line
            Err("`name` was empty; it must be nonempty.".into())
        );
    }
}
