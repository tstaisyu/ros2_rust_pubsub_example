cmake_minimum_required(VERSION 3.5)
project(ros2_rust_pubsub_example)

find_package(ament_cmake REQUIRED)

# 実際のRustビルドを呼び出すカスタムコマンド
add_custom_target(
    rust_build
    COMMAND cargo build --manifest-path ${CMAKE_CURRENT_SOURCE_DIR}/Cargo.toml
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
    COMMENT "Compiling Rust project"
)

# インストールルール
install(
    DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}/target/debug/
    DESTINATION lib/${PROJECT_NAME}
    FILES_MATCHING PATTERN "*"
)

ament_package()

