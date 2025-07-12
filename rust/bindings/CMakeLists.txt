set(BINDINGS_RS_DEPS
  stm32cubemx
)

foreach (DEP ${BINDINGS_RS_DEPS})
  # Add include directories for each dependency
  list(APPEND BINDINGS_RS_CLANG_ARGS $<LIST:TRANSFORM,$<TARGET_PROPERTY:${DEP},INTERFACE_INCLUDE_DIRECTORIES>,PREPEND,-I>)
  # Add compile definitions for each dependency
  list(APPEND BINDINGS_RS_CLANG_ARGS $<LIST:TRANSFORM,$<TARGET_PROPERTY:${DEP},INTERFACE_COMPILE_DEFINITIONS>,PREPEND,-D>)
endforeach(DEP)

add_custom_target(
  BINDINGS_RS
  ALL
  COMMAND_EXPAND_LISTS
  COMMENT
  "Generating Rust bindings for C code"
  COMMAND
  bindgen
  --use-core
  -o
  ${CMAKE_CURRENT_LIST_DIR}/bindings.rs
  ${CMAKE_CURRENT_LIST_DIR}/bindings.h
  --allowlist-item HAL_UART.*
  --allowlist-item HAL_Delay
  --allowlist-item huart2
  --allowlist-item HAL_MAX_DELAY
  --allowlist-item Error_Handler
  --
  ${BINDINGS_RS_CLANG_ARGS}
  --target=thumbv7em-none-eabihf
  DEPENDS
  ${BINDINGS_RS_DEPS}
)