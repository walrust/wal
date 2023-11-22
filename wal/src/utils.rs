#[cfg(debug_assertions)]
pub mod debug {
    use gloo::{
        console::{log, warn},
        dialogs,
    };

    #[inline]
    pub fn log(text: impl ToString) {
        log!(text.to_string())
    }

    #[inline]
    pub fn alert(text: impl ToString) {
        dialogs::alert(text.to_string().as_str())
    }

    #[inline]
    pub fn warn(text: impl ToString) {
        warn!(text.to_string().as_str())
    }
}

#[cfg(not(debug_assertions))]
pub mod debug {
    #[cfg(not(debug_assertions))]
    #[inline]
    pub fn log(_text: impl ToString) {}

    #[cfg(not(debug_assertions))]
    #[inline]
    pub fn alert(_text: impl ToString) {}

    #[cfg(not(debug_assertions))]
    #[inline]
    pub fn warn(_text: impl ToString) {}
}

pub mod any_utils {
    use std::{any::Any, mem};

    pub fn raw_memory_compare(a: &Box<dyn Any>, b: &Box<dyn Any>) -> bool {
        let size_a = mem::size_of_val(&**a);
        let size_b = mem::size_of_val(&**b);
        if size_a != size_b {
            return false;
        }

        unsafe {
            let a_ptr = &**a as *const dyn Any as *const u8;
            let b_ptr = &**b as *const dyn Any as *const u8;

            for i in 0..size_a {
                if *a_ptr.add(i) != *b_ptr.add(i) {
                    return false;
                }
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use std::any::Any;

    use super::any_utils::raw_memory_compare;

    #[test]
    fn raw_memory_compare_should_return_true_when_values_are_the_same() {
        // Arrange
        let val = 1;
        let a: Box<dyn Any> = Box::new(val);
        let b: Box<dyn Any> = Box::new(val);

        // Act
        let result = raw_memory_compare(&a, &b);

        // Assert
        assert!(result);
    }

    #[test]
    fn raw_memory_compare_should_return_false_when_values_are_not_the_same() {
        // Arrange
        let a: Box<dyn Any> = Box::new(1);
        let b: Box<dyn Any> = Box::new(2);

        // Act
        let result = raw_memory_compare(&a, &b);

        // Assert
        assert!(!result);
    }

    #[test]
    fn raw_memory_compare_should_return_false_when_values_are_not_the_same_type() {
        // Arrange
        let a: Box<dyn Any> = Box::new(1);
        let b: Box<dyn Any> = Box::new(1.0);

        // Act
        let result = raw_memory_compare(&a, &b);

        // Assert
        assert!(!result);
    }

    #[test]
    fn raw_memory_compare_should_return_false_when_values_are_not_the_same_size() {
        // Arrange
        let a: Box<dyn Any> = Box::new(vec![1, 2, 3]);
        let b: Box<dyn Any> = Box::new(vec![1, 2, 3, 4]);

        // Act
        let result = raw_memory_compare(&a, &b);

        // Assert
        assert!(!result);
    }

    #[test]
    fn raw_memory_compare_should_return_false_when_values_are_collections_not_the_same_type() {
        // Arrange
        let a: Box<dyn Any> = Box::new(vec![1]);
        let b: Box<dyn Any> = Box::new([1]);

        // Act
        let result = raw_memory_compare(&a, &b);

        // Assert
        assert!(!result);
    }
}
