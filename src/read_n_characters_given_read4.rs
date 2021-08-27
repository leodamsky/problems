/**
 * The read4 API is defined as.
 *     fn read4(&self,buf4: &mut [char]) -> i32;
 * You can call it using self.read4(buf4)
 */
impl Solution {
    fn read4(&self, _buf4: &mut [char]) -> i32 {
        todo!()
    }

    pub fn read(&self, buf: &mut [char], n: i32) -> i32 {
        let mut pointer = 0;
        let mut buf4 = ['\0'; 4];
        loop {
            let read = self.read4(&mut buf4);
            // LeetCode uses an outdated Rust version, where arrays are not iterators
            for c in buf4.iter().copied() {
                if c == '\0' || pointer as i32 == n {
                    break;
                }
                buf[pointer] = c;
                pointer += 1;
            }
            if read == 0 {
                break;
            }
            buf4 = ['\0'; 4];
        }
        pointer as i32
    }
}

struct Solution;
