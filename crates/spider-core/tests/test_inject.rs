use spider_core::invocation::Spider;
use spider_proc_macros::inject;

#[test]
fn test_inject_type() {
    let ty = Spider::new().expect("failed to create spider-node type");
    let dir = ty.objects();


}

#[inject]
fn task(#[bean(id = "hello")] name: String) {

}