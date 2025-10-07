fn fix_incorrect_order_2() {
    // super::back_of_house::cook_order();
    //super::super::outside::iu(); // X not ok. A function (or any item) can only access another private item directly
    // if that item’s defining module is on or above its own module chain.
    cook_order_2();
}

fn cook_order_2() {

}