use crate::models::{ProductDatabase, ProductWasChanged};

pub fn check_for_change(old: &ProductDatabase, new: &ProductDatabase) -> Vec<ProductWasChanged> {
    let mut changes: Vec<ProductWasChanged> = Vec::new();

    if new.available != old.available {
        if new.available == true {
            changes.push(ProductWasChanged::IsAvailable);
        } else {
            changes.push(ProductWasChanged::NoLongerAvailable);
        }
    }

    if new.price != old.price {
        changes.push(ProductWasChanged::PriceChange);
    }

    if new.variant_title != old.variant_title {
        changes.push(ProductWasChanged::VariantTitle);
    }

    if new.title != old.title {
        changes.push(ProductWasChanged::ProductTitle);
    }

    changes
}

pub fn change_text(
    change: ProductWasChanged,
    old: &ProductDatabase,
    new: &ProductDatabase,
) -> String {
    match change {
        ProductWasChanged::PriceChange => {
            format!("__**Price Change**__\n{} → {}", old.price, new.price)
        }
        ProductWasChanged::NoLongerAvailable => {
            format!("__**Available Change**__\n Is available: <:redTick:851441548994412614>")
        }
        ProductWasChanged::IsAvailable => {
            format!("__**Available Change**__\n Is available: <:greenTick:851441548922847262>")
        }
        ProductWasChanged::VariantTitle => {
            format!(
                "__**Variant Title Change**__\n{} → {}",
                old.variant_title, new.variant_title
            )
        }
        ProductWasChanged::ProductTitle => {
            format!(
                "__**Product Title Change**__\n{} → {}",
                old.title, new.title
            )
        }
    }
}
