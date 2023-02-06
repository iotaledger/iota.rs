{
  "targets": [
    {
      "target_name": "index",
      'defines': [
        "NAPI_VERSION=<(napi_build_version)",
      ],
      "win_delay_load_hook": "true",
    }
  ]
}