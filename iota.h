#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <new>

struct GetNodeInfoResponse {
  const char *app_name;
  uint32_t latest_milestone_index;
};

extern "C" {

GetNodeInfoResponse *get_node_info(const char *url);

const int8_t *iota_address_gen(const int8_t *seed, uint64_t index);

} // extern "C"
