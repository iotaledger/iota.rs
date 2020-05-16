#include <stdint.h>
#include <stdio.h>


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
 * @param[in] url The node URL
 * @return Response type of node information
 */
extern get_node_info_t *get_node_info(const char *url);

int main() {
    // Here we create a dummy seed.
    int8_t seed[243] = {0};
    // Generate the address in index 0.
    int8_t *address = iota_address_gen(seed, 0);
    printf("This address is generated from iota.rs:\n");
    for(int i = 0; i < 243; i++) {
        printf("%d ", address[i]);
    }

    printf("\nFollowing node information is retrieved from iota.rs:\n");
    // This is the IRI API call get_nod_info.
    get_node_info_t *node_info = get_node_info("https://nodes.comnet.thetangle.org");
    // We only define a few fields in the response struct. But this should give a glance how to use it.
    printf("Node name: %s\n", node_info->app_name);
    printf("Node version: %s\n", node_info->app_version);
    printf("Last milestone index: %d\n", node_info->latest_milestone_index);
    return 0;
}