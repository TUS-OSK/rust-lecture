fn main() {
    let a = 0;
    {
        let b = 1;
        {
            let c = 2;
        }
    }

    {
        let d = 3;
    }
}