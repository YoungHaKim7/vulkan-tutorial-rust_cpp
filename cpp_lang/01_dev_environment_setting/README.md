# Setting up a makefile project(vulkan)
- https://vulkan-tutorial.com/Development_environment


# .clang-format만들기

```bash
clang-format -style=WebKit -dump-config > .clang-format
```

- .clang-format fmt정렬
```bash
# Then run:

find . -regex '.*\.\(cpp\|hpp\|cc\|cxx\)' -exec clang-format -style=file -i {} \;
```
