SRC_PATH = ./Simple_Caro_wrapper.cpp
INC_PATH = ./Simple_Caro_wrapper.h

LIB_NAME = ../libSimple_Caro.a
OBJECTS = $(SRC_PATH:.cpp=.o)
BIND_INC = ../Simple_Caro.rs

all: $(LIB_NAME)

$(LIB_NAME): $(OBJECTS)
	ar rcs $@ $^

clean:
	rm -f $(OBJECTS) $(LIB_NAME)

bindgen:
	bindgen $(INC_PATH) -o $(BIND_INC)

.PHONY: all clean