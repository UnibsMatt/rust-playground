#include <cstdarg>
#include <cstddef>
#include <cstdint>
#include <cstdlib>
#include <ostream>


enum class CodecResponse {
  Ok = 0,
  EncodeError = -1,
  DecodeError = -2,
};

struct String;

struct Point {
  uint8_t ciao;
  const uint8_t *ciao2;
  int32_t ciao3;
};


extern "C" {

const uint8_t *add(String uno, uint8_t due);

CodecResponse deu(Point asd);

} // extern "C"
