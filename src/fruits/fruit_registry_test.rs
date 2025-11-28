//! FruitRegistry 属性测试

#[cfg(test)]
mod proptests {
    use proptest::prelude::*;
    use crate::fruits::{create_fruit_registry, FruitCategory};
    use ::rand::thread_rng;

    // **Feature: modular-migration, Property 6: Fruit Registry Unlock Condition**
    // *For any* snake length and fruit category, FruitRegistry::random_by_category
    // should only return fruits whose unlock_length <= snake_length.
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]

        #[test]
        fn prop_fruit_unlock_condition(
            snake_length in 1usize..50,
            category_idx in 0usize..4,
        ) {
            let registry = create_fruit_registry();
            let mut rng = thread_rng();

            let category = match category_idx {
                0 => FruitCategory::Normal,
                1 => FruitCategory::Trap,
                2 => FruitCategory::Power,
                _ => FruitCategory::Special,
            };

            // 尝试多次获取随机果实
            for _ in 0..10 {
                if let Some(fruit_id) = registry.random_by_category(category, snake_length, &mut rng) {
                    // 验证返回的果实满足解锁条件
                    if let Some(config) = registry.get_config(fruit_id) {
                        prop_assert!(
                            config.unlock_length <= snake_length,
                            "Fruit {} requires length {} but snake length is {}",
                            fruit_id,
                            config.unlock_length,
                            snake_length
                        );
                    }
                }
            }
        }
    }
}
