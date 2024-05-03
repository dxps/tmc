use nanoid::nanoid;

pub fn create_id() -> String {
    nanoid!(11)
}
