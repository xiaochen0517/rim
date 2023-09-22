// 测试代码
#[cfg(test)]
mod tests {
    use std::io::stdout;

    #[test]
    fn test_stdout() {
        // 获取当前时间 毫秒
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis();
        for index in 0..100000u64 {
            let stdout = stdout();
        }
        // 获取当前时间 毫秒
        let now2 = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis();
        println!("stdout: {}", now2 - now);
    }
}
