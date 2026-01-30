fn main() {
    env_logger::init();

    //do_it("--long-b=option sub", true);
    //do_it("-b=option sub", true);
    //do_it("-cb=abcd sub", true);
    //do_it("-cb abcd sub", true);
    //do_it("-cbabcd sub", true);
    //
    //do_it("-bc=abcd sub", true);

    //do_it("-", false);
    //do_it("-b a -", false); // test "once"
    //do_it("--", false);
    //do_it("sub -", false);
    //do_it("sub --", false);
    //do_it("-b", true);
    //do_it("sub -bmy_option", true);
    //
    //do_it("-bmy_option sub", true);
    //
    //do_it("-cbmy_option sub a1", true);
}
