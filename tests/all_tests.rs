mod infra;

// Your tests go here!
success_tests! {
    {
        name: sumrec,
        file: "selfrec_sumrec.snek",
        expected: "1000000000",
    },
    {
        name: evenodd,
        file: "mutrec_evenodd.snek",
        expected: "true",
    },
    {
        name: halve,
        file: "halve_tail.snek",
        expected: "1000000000",
    },
}

runtime_error_tests! {
    // integer overflow
 
}

static_error_tests! {

}
