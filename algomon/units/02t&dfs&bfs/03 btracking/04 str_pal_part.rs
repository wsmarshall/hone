fn is_palindrome(chars: &Vec<String>, left: usize, right: usize) -> bool {
    let mut l = left;
    let mut r = right;
    while (l < r) {
        if chars[left] != chars[right] {
                return false;
        }
        l+=1;
        r-=1;
    }
    true
}

fn dfs(n: usize, start_index: usize, chars: &Vec<&str>, 
    path: &mut Vec<String>, res: &mut &Vec<Vec<String>>) {
        
    if start_index >=n {
        res.push(path.clone());
        return;
    }
        
    for i in start_index..n {
        if !is_palindrome(path) {
            return;
        } else {
            path.push(chars[i]);
            dfs(n, start_index + 1, chars, path, res);
            path.pop();
        }
    }
        
}

fn partition(s: String) -> Vec<Vec<String>> {
    let mut path = Vec<String>;
    
    let mut chars: Vec<&str> = s.split("").collect();
    chars.remove(0);
    chars.pop();
   
    let n = chars.len();
    
    let mut res = Vec::<Vec::<String>>::new();
    
    dfs(n, 0, &chars, &mut path, &mut res);
    
    res
}