package main

func must(err error) {
    if err != nil {
        panic(err)
    }
}

func main() {
    day1()
}
