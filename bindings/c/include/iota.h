#include <stdint.h>

/**
 * @brief Initialize the iota service instance.
 *
 * @param[in] url The node URL
 */
extern void *iota_init(const char *url);

/**
 * @brief The data structure of the get node info response.
 *
 */
typedef struct GetNodeInfoResponse {
  /**
   * Name of IRI node.
   */
  const char *app_name;
  /**
   * Version of IRI node.
   */
  const char *app_version;
  /**
   * Index of the latest milestone.
   */
  uint32_t latest_milestone_index;
} get_node_info_t;

/**
 * @brief Generate IOTA address
 *
 * This is a raw function to generate address which is unchecked by any node.
 * This is just for development purpose and should not be used by users.
 *
 * @param[in] seed A 243 trits long IOTA seed.
 * @param[in] index Index of the address
 * @return A 243 trits long IOTA address.
 */
extern int8_t *iota_address_gen(const int8_t *seed, uint64_t index);


/**
 * @brief Returns information about connected node.
 *
 * @return Response type of node information
 */
extern get_node_info_t *iota_get_node_info();