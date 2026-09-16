pub fn mul(num1: &[u8], num2: &[u8]) -> Vec<u8> {
    let mut ans = vec![0; num1.len() + num2.len()];

    for (i, &x) in num1.iter().enumerate() {
        let mut carry = 0;

        for (j, &y) in num2.iter().enumerate() {
            let total = ans[i + j] + x * y + carry;
            ans[i + j] = total % 10;
            carry = total / 10;
        }

        ans[i + num2.len()] = carry;
    }

    normalize(ans)
}

pub fn add(num1: &[u8], num2: &[u8]) -> Vec<u8> {
    let len = num1.len().max(num2.len());
    let mut ans = Vec::with_capacity(len + 1);
    let mut carry = 0;

    for i in 0..len {
        let x = *num1.get(i).unwrap_or(&0);
        let y = *num2.get(i).unwrap_or(&0);
        let total = x + y + carry;

        ans.push(total % 10);
        carry = total / 10;
    }

    if carry != 0 {
        ans.push(carry);
    }

    normalize(ans)
}

fn normalize(mut d: Vec<u8>) -> Vec<u8> {
    while d.len() > 1 && d.last() == Some(&0) {
        d.pop();
    }
    if d.is_empty() {
        d.push(0);
    }
    d
}
