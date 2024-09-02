from itertools import product

def generate_swizzle_methods(components):
    methods = []
    for length in range(1, 5):  # Generate combinations up to length 4
        for swizzle in product(components, repeat=length):
            if length == 1:
                method_name = swizzle[0]
                return_type = "T"
                method_body = f"self.{swizzle[0]}"
                method = f"""
    pub fn {method_name}(&self) -> {return_type} {{
        return {method_body}
    }}
"""
            else:
                method_name = ''.join(swizzle)
                return_type = f"Vec{length}<T>"
                method_body = f"Vec{length}::new({', '.join(f'self.{c}' for c in swizzle)})"
                method = f"""
    pub fn {method_name}(&self) -> {return_type} {{
        return {method_body}
    }}
"""
            methods.append(method)
    return methods

def generate_vec_impl(components):
    methods = generate_swizzle_methods(components)
    impl = f"""
#[doc(hidden)]
impl<T: Copy> Vec{len(components)}<T> {{
    {'    '.join(methods)}
}}
"""
    return impl

def main():
    vec_types = {
        "Vec2": ["x", "y"],
        "Vec3": ["x", "y", "z"],
        "Vec4": ["x", "y", "z", "w"]
    }

    print("use crate::{Vec2, Vec3, Vec4};")
    for vec_type, components in vec_types.items():
        impl = generate_vec_impl(components)
        print(impl)

if __name__ == "__main__":
    main()