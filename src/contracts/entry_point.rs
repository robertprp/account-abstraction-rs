pub use entry_point::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types
)]
pub mod entry_point {
    #[rustfmt::skip]
    const __ABI: &str = "[\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"preOpGas\",\n          \"type\": \"uint256\"\n        },\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"paid\",\n          \"type\": \"uint256\"\n        },\n        {\n          \"internalType\": \"uint48\",\n          \"name\": \"validAfter\",\n          \"type\": \"uint48\"\n        },\n        {\n          \"internalType\": \"uint48\",\n          \"name\": \"validUntil\",\n          \"type\": \"uint48\"\n        },\n        {\n          \"internalType\": \"bool\",\n          \"name\": \"targetSuccess\",\n          \"type\": \"bool\"\n        },\n        {\n          \"internalType\": \"bytes\",\n          \"name\": \"targetResult\",\n          \"type\": \"bytes\"\n        }\n      ],\n      \"name\": \"ExecutionResult\",\n      \"type\": \"error\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"opIndex\",\n          \"type\": \"uint256\"\n        },\n        {\n          \"internalType\": \"string\",\n          \"name\": \"reason\",\n          \"type\": \"string\"\n        }\n      ],\n      \"name\": \"FailedOp\",\n      \"type\": \"error\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"sender\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"SenderAddressResult\",\n      \"type\": \"error\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"aggregator\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"SignatureValidationFailed\",\n      \"type\": \"error\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"components\": [\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"preOpGas\",\n              \"type\": \"uint256\"\n            },\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"prefund\",\n              \"type\": \"uint256\"\n            },\n            {\n              \"internalType\": \"bool\",\n              \"name\": \"sigFailed\",\n              \"type\": \"bool\"\n            },\n            {\n              \"internalType\": \"uint48\",\n              \"name\": \"validAfter\",\n              \"type\": \"uint48\"\n            },\n            {\n              \"internalType\": \"uint48\",\n              \"name\": \"validUntil\",\n              \"type\": \"uint48\"\n            },\n            {\n              \"internalType\": \"bytes\",\n              \"name\": \"paymasterContext\",\n              \"type\": \"bytes\"\n            }\n          ],\n          \"internalType\": \"struct IEntryPoint.ReturnInfo\",\n          \"name\": \"returnInfo\",\n          \"type\": \"tuple\"\n        },\n        {\n          \"components\": [\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"stake\",\n              \"type\": \"uint256\"\n            },\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"unstakeDelaySec\",\n              \"type\": \"uint256\"\n            }\n          ],\n          \"internalType\": \"struct IStakeManager.StakeInfo\",\n          \"name\": \"senderInfo\",\n          \"type\": \"tuple\"\n        },\n        {\n          \"components\": [\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"stake\",\n              \"type\": \"uint256\"\n            },\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"unstakeDelaySec\",\n              \"type\": \"uint256\"\n            }\n          ],\n          \"internalType\": \"struct IStakeManager.StakeInfo\",\n          \"name\": \"factoryInfo\",\n          \"type\": \"tuple\"\n        },\n        {\n          \"components\": [\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"stake\",\n              \"type\": \"uint256\"\n            },\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"unstakeDelaySec\",\n              \"type\": \"uint256\"\n            }\n          ],\n          \"internalType\": \"struct IStakeManager.StakeInfo\",\n          \"name\": \"paymasterInfo\",\n          \"type\": \"tuple\"\n        }\n      ],\n      \"name\": \"ValidationResult\",\n      \"type\": \"error\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"components\": [\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"preOpGas\",\n              \"type\": \"uint256\"\n            },\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"prefund\",\n              \"type\": \"uint256\"\n            },\n            {\n              \"internalType\": \"bool\",\n              \"name\": \"sigFailed\",\n              \"type\": \"bool\"\n            },\n            {\n              \"internalType\": \"uint48\",\n              \"name\": \"validAfter\",\n              \"type\": \"uint48\"\n            },\n            {\n              \"internalType\": \"uint48\",\n              \"name\": \"validUntil\",\n              \"type\": \"uint48\"\n            },\n            {\n              \"internalType\": \"bytes\",\n              \"name\": \"paymasterContext\",\n              \"type\": \"bytes\"\n            }\n          ],\n          \"internalType\": \"struct IEntryPoint.ReturnInfo\",\n          \"name\": \"returnInfo\",\n          \"type\": \"tuple\"\n        },\n        {\n          \"components\": [\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"stake\",\n              \"type\": \"uint256\"\n            },\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"unstakeDelaySec\",\n              \"type\": \"uint256\"\n            }\n          ],\n          \"internalType\": \"struct IStakeManager.StakeInfo\",\n          \"name\": \"senderInfo\",\n          \"type\": \"tuple\"\n        },\n        {\n          \"components\": [\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"stake\",\n              \"type\": \"uint256\"\n            },\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"unstakeDelaySec\",\n              \"type\": \"uint256\"\n            }\n          ],\n          \"internalType\": \"struct IStakeManager.StakeInfo\",\n          \"name\": \"factoryInfo\",\n          \"type\": \"tuple\"\n        },\n        {\n          \"components\": [\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"stake\",\n              \"type\": \"uint256\"\n            },\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"unstakeDelaySec\",\n              \"type\": \"uint256\"\n            }\n          ],\n          \"internalType\": \"struct IStakeManager.StakeInfo\",\n          \"name\": \"paymasterInfo\",\n          \"type\": \"tuple\"\n        },\n        {\n          \"components\": [\n            {\n              \"internalType\": \"address\",\n              \"name\": \"aggregator\",\n              \"type\": \"address\"\n            },\n            {\n              \"components\": [\n                {\n                  \"internalType\": \"uint256\",\n                  \"name\": \"stake\",\n                  \"type\": \"uint256\"\n                },\n                {\n                  \"internalType\": \"uint256\",\n                  \"name\": \"unstakeDelaySec\",\n                  \"type\": \"uint256\"\n                }\n              ],\n              \"internalType\": \"struct IStakeManager.StakeInfo\",\n              \"name\": \"stakeInfo\",\n              \"type\": \"tuple\"\n            }\n          ],\n          \"internalType\": \"struct IEntryPoint.AggregatorStakeInfo\",\n          \"name\": \"aggregatorInfo\",\n          \"type\": \"tuple\"\n        }\n      ],\n      \"name\": \"ValidationResultWithAggregation\",\n      \"type\": \"error\"\n    },\n    {\n      \"anonymous\": false,\n      \"inputs\": [\n        {\n          \"indexed\": true,\n          \"internalType\": \"bytes32\",\n          \"name\": \"userOpHash\",\n          \"type\": \"bytes32\"\n        },\n        {\n          \"indexed\": true,\n          \"internalType\": \"address\",\n          \"name\": \"sender\",\n          \"type\": \"address\"\n        },\n        {\n          \"indexed\": false,\n          \"internalType\": \"address\",\n          \"name\": \"factory\",\n          \"type\": \"address\"\n        },\n        {\n          \"indexed\": false,\n          \"internalType\": \"address\",\n          \"name\": \"paymaster\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"AccountDeployed\",\n      \"type\": \"event\"\n    },\n    {\n      \"anonymous\": false,\n      \"inputs\": [],\n      \"name\": \"BeforeExecution\",\n      \"type\": \"event\"\n    },\n    {\n      \"anonymous\": false,\n      \"inputs\": [\n        {\n          \"indexed\": true,\n          \"internalType\": \"address\",\n          \"name\": \"account\",\n          \"type\": \"address\"\n        },\n        {\n          \"indexed\": false,\n          \"internalType\": \"uint256\",\n          \"name\": \"totalDeposit\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"name\": \"Deposited\",\n      \"type\": \"event\"\n    },\n    {\n      \"anonymous\": false,\n      \"inputs\": [\n        {\n          \"indexed\": true,\n          \"internalType\": \"address\",\n          \"name\": \"aggregator\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"SignatureAggregatorChanged\",\n      \"type\": \"event\"\n    },\n    {\n      \"anonymous\": false,\n      \"inputs\": [\n        {\n          \"indexed\": true,\n          \"internalType\": \"address\",\n          \"name\": \"account\",\n          \"type\": \"address\"\n        },\n        {\n          \"indexed\": false,\n          \"internalType\": \"uint256\",\n          \"name\": \"totalStaked\",\n          \"type\": \"uint256\"\n        },\n        {\n          \"indexed\": false,\n          \"internalType\": \"uint256\",\n          \"name\": \"unstakeDelaySec\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"name\": \"StakeLocked\",\n      \"type\": \"event\"\n    },\n    {\n      \"anonymous\": false,\n      \"inputs\": [\n        {\n          \"indexed\": true,\n          \"internalType\": \"address\",\n          \"name\": \"account\",\n          \"type\": \"address\"\n        },\n        {\n          \"indexed\": false,\n          \"internalType\": \"uint256\",\n          \"name\": \"withdrawTime\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"name\": \"StakeUnlocked\",\n      \"type\": \"event\"\n    },\n    {\n      \"anonymous\": false,\n      \"inputs\": [\n        {\n          \"indexed\": true,\n          \"internalType\": \"address\",\n          \"name\": \"account\",\n          \"type\": \"address\"\n        },\n        {\n          \"indexed\": false,\n          \"internalType\": \"address\",\n          \"name\": \"withdrawAddress\",\n          \"type\": \"address\"\n        },\n        {\n          \"indexed\": false,\n          \"internalType\": \"uint256\",\n          \"name\": \"amount\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"name\": \"StakeWithdrawn\",\n      \"type\": \"event\"\n    },\n    {\n      \"anonymous\": false,\n      \"inputs\": [\n        {\n          \"indexed\": true,\n          \"internalType\": \"bytes32\",\n          \"name\": \"userOpHash\",\n          \"type\": \"bytes32\"\n        },\n        {\n          \"indexed\": true,\n          \"internalType\": \"address\",\n          \"name\": \"sender\",\n          \"type\": \"address\"\n        },\n        {\n          \"indexed\": true,\n          \"internalType\": \"address\",\n          \"name\": \"paymaster\",\n          \"type\": \"address\"\n        },\n        {\n          \"indexed\": false,\n          \"internalType\": \"uint256\",\n          \"name\": \"nonce\",\n          \"type\": \"uint256\"\n        },\n        {\n          \"indexed\": false,\n          \"internalType\": \"bool\",\n          \"name\": \"success\",\n          \"type\": \"bool\"\n        },\n        {\n          \"indexed\": false,\n          \"internalType\": \"uint256\",\n          \"name\": \"actualGasCost\",\n          \"type\": \"uint256\"\n        },\n        {\n          \"indexed\": false,\n          \"internalType\": \"uint256\",\n          \"name\": \"actualGasUsed\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"name\": \"UserOperationEvent\",\n      \"type\": \"event\"\n    },\n    {\n      \"anonymous\": false,\n      \"inputs\": [\n        {\n          \"indexed\": true,\n          \"internalType\": \"bytes32\",\n          \"name\": \"userOpHash\",\n          \"type\": \"bytes32\"\n        },\n        {\n          \"indexed\": true,\n          \"internalType\": \"address\",\n          \"name\": \"sender\",\n          \"type\": \"address\"\n        },\n        {\n          \"indexed\": false,\n          \"internalType\": \"uint256\",\n          \"name\": \"nonce\",\n          \"type\": \"uint256\"\n        },\n        {\n          \"indexed\": false,\n          \"internalType\": \"bytes\",\n          \"name\": \"revertReason\",\n          \"type\": \"bytes\"\n        }\n      ],\n      \"name\": \"UserOperationRevertReason\",\n      \"type\": \"event\"\n    },\n    {\n      \"anonymous\": false,\n      \"inputs\": [\n        {\n          \"indexed\": true,\n          \"internalType\": \"address\",\n          \"name\": \"account\",\n          \"type\": \"address\"\n        },\n        {\n          \"indexed\": false,\n          \"internalType\": \"address\",\n          \"name\": \"withdrawAddress\",\n          \"type\": \"address\"\n        },\n        {\n          \"indexed\": false,\n          \"internalType\": \"uint256\",\n          \"name\": \"amount\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"name\": \"Withdrawn\",\n      \"type\": \"event\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"SIG_VALIDATION_FAILED\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"bytes\",\n          \"name\": \"initCode\",\n          \"type\": \"bytes\"\n        },\n        {\n          \"internalType\": \"address\",\n          \"name\": \"sender\",\n          \"type\": \"address\"\n        },\n        {\n          \"internalType\": \"bytes\",\n          \"name\": \"paymasterAndData\",\n          \"type\": \"bytes\"\n        }\n      ],\n      \"name\": \"_validateSenderAndPaymaster\",\n      \"outputs\": [],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"uint32\",\n          \"name\": \"unstakeDelaySec\",\n          \"type\": \"uint32\"\n        }\n      ],\n      \"name\": \"addStake\",\n      \"outputs\": [],\n      \"stateMutability\": \"payable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"account\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"balanceOf\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"account\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"depositTo\",\n      \"outputs\": [],\n      \"stateMutability\": \"payable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"deposits\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint112\",\n          \"name\": \"deposit\",\n          \"type\": \"uint112\"\n        },\n        {\n          \"internalType\": \"bool\",\n          \"name\": \"staked\",\n          \"type\": \"bool\"\n        },\n        {\n          \"internalType\": \"uint112\",\n          \"name\": \"stake\",\n          \"type\": \"uint112\"\n        },\n        {\n          \"internalType\": \"uint32\",\n          \"name\": \"unstakeDelaySec\",\n          \"type\": \"uint32\"\n        },\n        {\n          \"internalType\": \"uint48\",\n          \"name\": \"withdrawTime\",\n          \"type\": \"uint48\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"account\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"getDepositInfo\",\n      \"outputs\": [\n        {\n          \"components\": [\n            {\n              \"internalType\": \"uint112\",\n              \"name\": \"deposit\",\n              \"type\": \"uint112\"\n            },\n            {\n              \"internalType\": \"bool\",\n              \"name\": \"staked\",\n              \"type\": \"bool\"\n            },\n            {\n              \"internalType\": \"uint112\",\n              \"name\": \"stake\",\n              \"type\": \"uint112\"\n            },\n            {\n              \"internalType\": \"uint32\",\n              \"name\": \"unstakeDelaySec\",\n              \"type\": \"uint32\"\n            },\n            {\n              \"internalType\": \"uint48\",\n              \"name\": \"withdrawTime\",\n              \"type\": \"uint48\"\n            }\n          ],\n          \"internalType\": \"struct IStakeManager.DepositInfo\",\n          \"name\": \"info\",\n          \"type\": \"tuple\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"sender\",\n          \"type\": \"address\"\n        },\n        {\n          \"internalType\": \"uint192\",\n          \"name\": \"key\",\n          \"type\": \"uint192\"\n        }\n      ],\n      \"name\": \"getNonce\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"nonce\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"bytes\",\n          \"name\": \"initCode\",\n          \"type\": \"bytes\"\n        }\n      ],\n      \"name\": \"getSenderAddress\",\n      \"outputs\": [],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"components\": [\n            {\n              \"internalType\": \"address\",\n              \"name\": \"sender\",\n              \"type\": \"address\"\n            },\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"nonce\",\n              \"type\": \"uint256\"\n            },\n            {\n              \"internalType\": \"bytes\",\n              \"name\": \"initCode\",\n              \"type\": \"bytes\"\n            },\n            {\n              \"internalType\": \"bytes\",\n              \"name\": \"callData\",\n              \"type\": \"bytes\"\n            },\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"callGasLimit\",\n              \"type\": \"uint256\"\n            },\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"verificationGasLimit\",\n              \"type\": \"uint256\"\n            },\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"preVerificationGas\",\n              \"type\": \"uint256\"\n            },\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"maxFeePerGas\",\n              \"type\": \"uint256\"\n            },\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"maxPriorityFeePerGas\",\n              \"type\": \"uint256\"\n            },\n            {\n              \"internalType\": \"bytes\",\n              \"name\": \"paymasterAndData\",\n              \"type\": \"bytes\"\n            },\n            {\n              \"internalType\": \"bytes\",\n              \"name\": \"signature\",\n              \"type\": \"bytes\"\n            }\n          ],\n          \"internalType\": \"struct UserOperation\",\n          \"name\": \"userOp\",\n          \"type\": \"tuple\"\n        }\n      ],\n      \"name\": \"getUserOpHash\",\n      \"outputs\": [\n        {\n          \"internalType\": \"bytes32\",\n          \"name\": \"\",\n          \"type\": \"bytes32\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"components\": [\n            {\n              \"components\": [\n                {\n                  \"internalType\": \"address\",\n                  \"name\": \"sender\",\n                  \"type\": \"address\"\n                },\n                {\n                  \"internalType\": \"uint256\",\n                  \"name\": \"nonce\",\n                  \"type\": \"uint256\"\n                },\n                {\n                  \"internalType\": \"bytes\",\n                  \"name\": \"initCode\",\n                  \"type\": \"bytes\"\n                },\n                {\n                  \"internalType\": \"bytes\",\n                  \"name\": \"callData\",\n                  \"type\": \"bytes\"\n                },\n                {\n                  \"internalType\": \"uint256\",\n                  \"name\": \"callGasLimit\",\n                  \"type\": \"uint256\"\n                },\n                {\n                  \"internalType\": \"uint256\",\n                  \"name\": \"verificationGasLimit\",\n                  \"type\": \"uint256\"\n                },\n                {\n                  \"internalType\": \"uint256\",\n                  \"name\": \"preVerificationGas\",\n                  \"type\": \"uint256\"\n                },\n                {\n                  \"internalType\": \"uint256\",\n                  \"name\": \"maxFeePerGas\",\n                  \"type\": \"uint256\"\n                },\n                {\n                  \"internalType\": \"uint256\",\n                  \"name\": \"maxPriorityFeePerGas\",\n                  \"type\": \"uint256\"\n                },\n                {\n                  \"internalType\": \"bytes\",\n                  \"name\": \"paymasterAndData\",\n                  \"type\": \"bytes\"\n                },\n                {\n                  \"internalType\": \"bytes\",\n                  \"name\": \"signature\",\n                  \"type\": \"bytes\"\n                }\n              ],\n              \"internalType\": \"struct UserOperation[]\",\n              \"name\": \"userOps\",\n              \"type\": \"tuple[]\"\n            },\n            {\n              \"internalType\": \"contract IAggregator\",\n              \"name\": \"aggregator\",\n              \"type\": \"address\"\n            },\n            {\n              \"internalType\": \"bytes\",\n              \"name\": \"signature\",\n              \"type\": \"bytes\"\n            }\n          ],\n          \"internalType\": \"struct IEntryPoint.UserOpsPerAggregator[]\",\n          \"name\": \"opsPerAggregator\",\n          \"type\": \"tuple[]\"\n        },\n        {\n          \"internalType\": \"address payable\",\n          \"name\": \"beneficiary\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"handleAggregatedOps\",\n      \"outputs\": [],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"components\": [\n            {\n              \"internalType\": \"address\",\n              \"name\": \"sender\",\n              \"type\": \"address\"\n            },\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"nonce\",\n              \"type\": \"uint256\"\n            },\n            {\n              \"internalType\": \"bytes\",\n              \"name\": \"initCode\",\n              \"type\": \"bytes\"\n            },\n            {\n              \"internalType\": \"bytes\",\n              \"name\": \"callData\",\n              \"type\": \"bytes\"\n            },\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"callGasLimit\",\n              \"type\": \"uint256\"\n            },\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"verificationGasLimit\",\n              \"type\": \"uint256\"\n            },\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"preVerificationGas\",\n              \"type\": \"uint256\"\n            },\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"maxFeePerGas\",\n              \"type\": \"uint256\"\n            },\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"maxPriorityFeePerGas\",\n              \"type\": \"uint256\"\n            },\n            {\n              \"internalType\": \"bytes\",\n              \"name\": \"paymasterAndData\",\n              \"type\": \"bytes\"\n            },\n            {\n              \"internalType\": \"bytes\",\n              \"name\": \"signature\",\n              \"type\": \"bytes\"\n            }\n          ],\n          \"internalType\": \"struct UserOperation[]\",\n          \"name\": \"ops\",\n          \"type\": \"tuple[]\"\n        },\n        {\n          \"internalType\": \"address payable\",\n          \"name\": \"beneficiary\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"handleOps\",\n      \"outputs\": [],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"uint192\",\n          \"name\": \"key\",\n          \"type\": \"uint192\"\n        }\n      ],\n      \"name\": \"incrementNonce\",\n      \"outputs\": [],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"bytes\",\n          \"name\": \"callData\",\n          \"type\": \"bytes\"\n        },\n        {\n          \"components\": [\n            {\n              \"components\": [\n                {\n                  \"internalType\": \"address\",\n                  \"name\": \"sender\",\n                  \"type\": \"address\"\n                },\n                {\n                  \"internalType\": \"uint256\",\n                  \"name\": \"nonce\",\n                  \"type\": \"uint256\"\n                },\n                {\n                  \"internalType\": \"uint256\",\n                  \"name\": \"callGasLimit\",\n                  \"type\": \"uint256\"\n                },\n                {\n                  \"internalType\": \"uint256\",\n                  \"name\": \"verificationGasLimit\",\n                  \"type\": \"uint256\"\n                },\n                {\n                  \"internalType\": \"uint256\",\n                  \"name\": \"preVerificationGas\",\n                  \"type\": \"uint256\"\n                },\n                {\n                  \"internalType\": \"address\",\n                  \"name\": \"paymaster\",\n                  \"type\": \"address\"\n                },\n                {\n                  \"internalType\": \"uint256\",\n                  \"name\": \"maxFeePerGas\",\n                  \"type\": \"uint256\"\n                },\n                {\n                  \"internalType\": \"uint256\",\n                  \"name\": \"maxPriorityFeePerGas\",\n                  \"type\": \"uint256\"\n                }\n              ],\n              \"internalType\": \"struct EntryPoint.MemoryUserOp\",\n              \"name\": \"mUserOp\",\n              \"type\": \"tuple\"\n            },\n            {\n              \"internalType\": \"bytes32\",\n              \"name\": \"userOpHash\",\n              \"type\": \"bytes32\"\n            },\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"prefund\",\n              \"type\": \"uint256\"\n            },\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"contextOffset\",\n              \"type\": \"uint256\"\n            },\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"preOpGas\",\n              \"type\": \"uint256\"\n            }\n          ],\n          \"internalType\": \"struct EntryPoint.UserOpInfo\",\n          \"name\": \"opInfo\",\n          \"type\": \"tuple\"\n        },\n        {\n          \"internalType\": \"bytes\",\n          \"name\": \"context\",\n          \"type\": \"bytes\"\n        }\n      ],\n      \"name\": \"innerHandleOp\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"actualGasCost\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"\",\n          \"type\": \"address\"\n        },\n        {\n          \"internalType\": \"uint192\",\n          \"name\": \"\",\n          \"type\": \"uint192\"\n        }\n      ],\n      \"name\": \"nonceSequenceNumber\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"components\": [\n            {\n              \"internalType\": \"address\",\n              \"name\": \"sender\",\n              \"type\": \"address\"\n            },\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"nonce\",\n              \"type\": \"uint256\"\n            },\n            {\n              \"internalType\": \"bytes\",\n              \"name\": \"initCode\",\n              \"type\": \"bytes\"\n            },\n            {\n              \"internalType\": \"bytes\",\n              \"name\": \"callData\",\n              \"type\": \"bytes\"\n            },\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"callGasLimit\",\n              \"type\": \"uint256\"\n            },\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"verificationGasLimit\",\n              \"type\": \"uint256\"\n            },\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"preVerificationGas\",\n              \"type\": \"uint256\"\n            },\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"maxFeePerGas\",\n              \"type\": \"uint256\"\n            },\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"maxPriorityFeePerGas\",\n              \"type\": \"uint256\"\n            },\n            {\n              \"internalType\": \"bytes\",\n              \"name\": \"paymasterAndData\",\n              \"type\": \"bytes\"\n            },\n            {\n              \"internalType\": \"bytes\",\n              \"name\": \"signature\",\n              \"type\": \"bytes\"\n            }\n          ],\n          \"internalType\": \"struct UserOperation\",\n          \"name\": \"op\",\n          \"type\": \"tuple\"\n        },\n        {\n          \"internalType\": \"address\",\n          \"name\": \"target\",\n          \"type\": \"address\"\n        },\n        {\n          \"internalType\": \"bytes\",\n          \"name\": \"targetCallData\",\n          \"type\": \"bytes\"\n        }\n      ],\n      \"name\": \"simulateHandleOp\",\n      \"outputs\": [],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"components\": [\n            {\n              \"internalType\": \"address\",\n              \"name\": \"sender\",\n              \"type\": \"address\"\n            },\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"nonce\",\n              \"type\": \"uint256\"\n            },\n            {\n              \"internalType\": \"bytes\",\n              \"name\": \"initCode\",\n              \"type\": \"bytes\"\n            },\n            {\n              \"internalType\": \"bytes\",\n              \"name\": \"callData\",\n              \"type\": \"bytes\"\n            },\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"callGasLimit\",\n              \"type\": \"uint256\"\n            },\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"verificationGasLimit\",\n              \"type\": \"uint256\"\n            },\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"preVerificationGas\",\n              \"type\": \"uint256\"\n            },\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"maxFeePerGas\",\n              \"type\": \"uint256\"\n            },\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"maxPriorityFeePerGas\",\n              \"type\": \"uint256\"\n            },\n            {\n              \"internalType\": \"bytes\",\n              \"name\": \"paymasterAndData\",\n              \"type\": \"bytes\"\n            },\n            {\n              \"internalType\": \"bytes\",\n              \"name\": \"signature\",\n              \"type\": \"bytes\"\n            }\n          ],\n          \"internalType\": \"struct UserOperation\",\n          \"name\": \"userOp\",\n          \"type\": \"tuple\"\n        }\n      ],\n      \"name\": \"simulateValidation\",\n      \"outputs\": [],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"unlockStake\",\n      \"outputs\": [],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"address payable\",\n          \"name\": \"withdrawAddress\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"withdrawStake\",\n      \"outputs\": [],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"address payable\",\n          \"name\": \"withdrawAddress\",\n          \"type\": \"address\"\n        },\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"withdrawAmount\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"name\": \"withdrawTo\",\n      \"outputs\": [],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"stateMutability\": \"payable\",\n      \"type\": \"receive\"\n    }\n  ]";
    ///The parsed JSON ABI of the contract.
    pub static ENTRYPOINT_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(|| {
            ::ethers::core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid")
        });
    pub struct EntryPoint<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for EntryPoint<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for EntryPoint<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for EntryPoint<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for EntryPoint<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(EntryPoint))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> EntryPoint<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                ENTRYPOINT_ABI.clone(),
                client,
            ))
        }
        ///Calls the contract's `SIG_VALIDATION_FAILED` (0x8f41ec5a) function
        pub fn sig_validation_failed(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([143, 65, 236, 90], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `_validateSenderAndPaymaster` (0x957122ab) function
        pub fn validate_sender_and_paymaster(
            &self,
            init_code: ::ethers::core::types::Bytes,
            sender: ::ethers::core::types::Address,
            paymaster_and_data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([149, 113, 34, 171], (init_code, sender, paymaster_and_data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `addStake` (0x0396cb60) function
        pub fn add_stake(
            &self,
            unstake_delay_sec: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([3, 150, 203, 96], unstake_delay_sec)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `balanceOf` (0x70a08231) function
        pub fn balance_of(
            &self,
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([112, 160, 130, 49], account)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `depositTo` (0xb760faf9) function
        pub fn deposit_to(
            &self,
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([183, 96, 250, 249], account)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `deposits` (0xfc7e286d) function
        pub fn deposits(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, (u128, bool, u128, u32, u64)> {
            self.0
                .method_hash([252, 126, 40, 109], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getDepositInfo` (0x5287ce12) function
        pub fn get_deposit_info(
            &self,
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, DepositInfo> {
            self.0
                .method_hash([82, 135, 206, 18], account)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getNonce` (0x35567e1a) function
        pub fn get_nonce(
            &self,
            sender: ::ethers::core::types::Address,
            key: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([53, 86, 126, 26], (sender, key))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getSenderAddress` (0x9b249f69) function
        pub fn get_sender_address(
            &self,
            init_code: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([155, 36, 159, 105], init_code)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getUserOpHash` (0xa6193531) function
        pub fn get_user_op_hash(
            &self,
            user_op: UserOperation,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([166, 25, 53, 49], (user_op,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `handleAggregatedOps` (0x4b1d7cf5) function
        pub fn handle_aggregated_ops(
            &self,
            ops_per_aggregator: ::std::vec::Vec<UserOpsPerAggregator>,
            beneficiary: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([75, 29, 124, 245], (ops_per_aggregator, beneficiary))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `handleOps` (0x1fad948c) function
        pub fn handle_ops(
            &self,
            ops: ::std::vec::Vec<UserOperation>,
            beneficiary: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([31, 173, 148, 140], (ops, beneficiary))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `incrementNonce` (0x0bd28e3b) function
        pub fn increment_nonce(
            &self,
            key: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([11, 210, 142, 59], key)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `innerHandleOp` (0x1d732756) function
        pub fn inner_handle_op(
            &self,
            call_data: ::ethers::core::types::Bytes,
            op_info: UserOpInfo,
            context: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([29, 115, 39, 86], (call_data, op_info, context))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `nonceSequenceNumber` (0x1b2e01b8) function
        pub fn nonce_sequence_number(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([27, 46, 1, 184], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `simulateHandleOp` (0xd6383f94) function
        pub fn simulate_handle_op(
            &self,
            op: UserOperation,
            target: ::ethers::core::types::Address,
            target_call_data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([214, 56, 63, 148], (op, target, target_call_data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `simulateValidation` (0xee219423) function
        pub fn simulate_validation(
            &self,
            user_op: UserOperation,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([238, 33, 148, 35], (user_op,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `unlockStake` (0xbb9fe6bf) function
        pub fn unlock_stake(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([187, 159, 230, 191], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `withdrawStake` (0xc23a5cea) function
        pub fn withdraw_stake(
            &self,
            withdraw_address: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([194, 58, 92, 234], withdraw_address)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `withdrawTo` (0x205c2878) function
        pub fn withdraw_to(
            &self,
            withdraw_address: ::ethers::core::types::Address,
            withdraw_amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([32, 92, 40, 120], (withdraw_address, withdraw_amount))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `AccountDeployed` event
        pub fn account_deployed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, AccountDeployedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `BeforeExecution` event
        pub fn before_execution_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, BeforeExecutionFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `Deposited` event
        pub fn deposited_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, DepositedFilter> {
            self.0.event()
        }
        ///Gets the contract's `SignatureAggregatorChanged` event
        pub fn signature_aggregator_changed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            SignatureAggregatorChangedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `StakeLocked` event
        pub fn stake_locked_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, StakeLockedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `StakeUnlocked` event
        pub fn stake_unlocked_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, StakeUnlockedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `StakeWithdrawn` event
        pub fn stake_withdrawn_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, StakeWithdrawnFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `UserOperationEvent` event
        pub fn user_operation_event_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, UserOperationEventFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `UserOperationRevertReason` event
        pub fn user_operation_revert_reason_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            UserOperationRevertReasonFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Withdrawn` event
        pub fn withdrawn_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, WithdrawnFilter> {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, EntryPointEvents> {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>> for EntryPoint<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `ExecutionResult` with signature `ExecutionResult(uint256,uint256,uint48,uint48,bool,bytes)` and selector `0x8b7ac980`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(
        name = "ExecutionResult",
        abi = "ExecutionResult(uint256,uint256,uint48,uint48,bool,bytes)"
    )]
    pub struct ExecutionResult {
        pub pre_op_gas: ::ethers::core::types::U256,
        pub paid: ::ethers::core::types::U256,
        pub valid_after: u64,
        pub valid_until: u64,
        pub target_success: bool,
        pub target_result: ::ethers::core::types::Bytes,
    }
    ///Custom Error type `FailedOp` with signature `FailedOp(uint256,string)` and selector `0x220266b6`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "FailedOp", abi = "FailedOp(uint256,string)")]
    pub struct FailedOp {
        pub op_index: ::ethers::core::types::U256,
        pub reason: ::std::string::String,
    }
    ///Custom Error type `SenderAddressResult` with signature `SenderAddressResult(address)` and selector `0x6ca7b806`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "SenderAddressResult", abi = "SenderAddressResult(address)")]
    pub struct SenderAddressResult {
        pub sender: ::ethers::core::types::Address,
    }
    ///Custom Error type `SignatureValidationFailed` with signature `SignatureValidationFailed(address)` and selector `0x86a9f750`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(
        name = "SignatureValidationFailed",
        abi = "SignatureValidationFailed(address)"
    )]
    pub struct SignatureValidationFailed {
        pub aggregator: ::ethers::core::types::Address,
    }
    ///Custom Error type `ValidationResult` with signature `ValidationResult((uint256,uint256,bool,uint48,uint48,bytes),(uint256,uint256),(uint256,uint256),(uint256,uint256))` and selector `0xe0cff05f`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(
        name = "ValidationResult",
        abi = "ValidationResult((uint256,uint256,bool,uint48,uint48,bytes),(uint256,uint256),(uint256,uint256),(uint256,uint256))"
    )]
    pub struct ValidationResult {
        pub return_info: (
            ::ethers::core::types::U256,
            ::ethers::core::types::U256,
            bool,
            u64,
            u64,
            ::ethers::core::types::Bytes,
        ),
        pub sender_info: (::ethers::core::types::U256, ::ethers::core::types::U256),
        pub factory_info: (::ethers::core::types::U256, ::ethers::core::types::U256),
        pub paymaster_info: (::ethers::core::types::U256, ::ethers::core::types::U256),
    }
    ///Custom Error type `ValidationResultWithAggregation` with signature `ValidationResultWithAggregation((uint256,uint256,bool,uint48,uint48,bytes),(uint256,uint256),(uint256,uint256),(uint256,uint256),(address,(uint256,uint256)))` and selector `0xfaecb4e4`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(
        name = "ValidationResultWithAggregation",
        abi = "ValidationResultWithAggregation((uint256,uint256,bool,uint48,uint48,bytes),(uint256,uint256),(uint256,uint256),(uint256,uint256),(address,(uint256,uint256)))"
    )]
    pub struct ValidationResultWithAggregation {
        pub return_info: (
            ::ethers::core::types::U256,
            ::ethers::core::types::U256,
            bool,
            u64,
            u64,
            ::ethers::core::types::Bytes,
        ),
        pub sender_info: (::ethers::core::types::U256, ::ethers::core::types::U256),
        pub factory_info: (::ethers::core::types::U256, ::ethers::core::types::U256),
        pub paymaster_info: (::ethers::core::types::U256, ::ethers::core::types::U256),
        pub aggregator_info: (
            ::ethers::core::types::Address,
            (::ethers::core::types::U256, ::ethers::core::types::U256),
        ),
    }
    ///Container type for all of the contract's custom errors
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum EntryPointErrors {
        ExecutionResult(ExecutionResult),
        FailedOp(FailedOp),
        SenderAddressResult(SenderAddressResult),
        SignatureValidationFailed(SignatureValidationFailed),
        ValidationResult(ValidationResult),
        ValidationResultWithAggregation(ValidationResultWithAggregation),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for EntryPointErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) =
                <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <ExecutionResult as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ExecutionResult(decoded));
            }
            if let Ok(decoded) = <FailedOp as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::FailedOp(decoded));
            }
            if let Ok(decoded) =
                <SenderAddressResult as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SenderAddressResult(decoded));
            }
            if let Ok(decoded) =
                <SignatureValidationFailed as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SignatureValidationFailed(decoded));
            }
            if let Ok(decoded) = <ValidationResult as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ValidationResult(decoded));
            }
            if let Ok(decoded) =
                <ValidationResultWithAggregation as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ValidationResultWithAggregation(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for EntryPointErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::ExecutionResult(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::FailedOp(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SenderAddressResult(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SignatureValidationFailed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ValidationResult(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ValidationResultWithAggregation(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for EntryPointErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <ExecutionResult as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <FailedOp as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <SenderAddressResult as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <SignatureValidationFailed as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ValidationResult as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ValidationResultWithAggregation as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for EntryPointErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ExecutionResult(element) => ::core::fmt::Display::fmt(element, f),
                Self::FailedOp(element) => ::core::fmt::Display::fmt(element, f),
                Self::SenderAddressResult(element) => ::core::fmt::Display::fmt(element, f),
                Self::SignatureValidationFailed(element) => ::core::fmt::Display::fmt(element, f),
                Self::ValidationResult(element) => ::core::fmt::Display::fmt(element, f),
                Self::ValidationResultWithAggregation(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for EntryPointErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<ExecutionResult> for EntryPointErrors {
        fn from(value: ExecutionResult) -> Self {
            Self::ExecutionResult(value)
        }
    }
    impl ::core::convert::From<FailedOp> for EntryPointErrors {
        fn from(value: FailedOp) -> Self {
            Self::FailedOp(value)
        }
    }
    impl ::core::convert::From<SenderAddressResult> for EntryPointErrors {
        fn from(value: SenderAddressResult) -> Self {
            Self::SenderAddressResult(value)
        }
    }
    impl ::core::convert::From<SignatureValidationFailed> for EntryPointErrors {
        fn from(value: SignatureValidationFailed) -> Self {
            Self::SignatureValidationFailed(value)
        }
    }
    impl ::core::convert::From<ValidationResult> for EntryPointErrors {
        fn from(value: ValidationResult) -> Self {
            Self::ValidationResult(value)
        }
    }
    impl ::core::convert::From<ValidationResultWithAggregation> for EntryPointErrors {
        fn from(value: ValidationResultWithAggregation) -> Self {
            Self::ValidationResultWithAggregation(value)
        }
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(
        name = "AccountDeployed",
        abi = "AccountDeployed(bytes32,address,address,address)"
    )]
    pub struct AccountDeployedFilter {
        #[ethevent(indexed)]
        pub user_op_hash: [u8; 32],
        #[ethevent(indexed)]
        pub sender: ::ethers::core::types::Address,
        pub factory: ::ethers::core::types::Address,
        pub paymaster: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "BeforeExecution", abi = "BeforeExecution()")]
    pub struct BeforeExecutionFilter;
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "Deposited", abi = "Deposited(address,uint256)")]
    pub struct DepositedFilter {
        #[ethevent(indexed)]
        pub account: ::ethers::core::types::Address,
        pub total_deposit: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(
        name = "SignatureAggregatorChanged",
        abi = "SignatureAggregatorChanged(address)"
    )]
    pub struct SignatureAggregatorChangedFilter {
        #[ethevent(indexed)]
        pub aggregator: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "StakeLocked", abi = "StakeLocked(address,uint256,uint256)")]
    pub struct StakeLockedFilter {
        #[ethevent(indexed)]
        pub account: ::ethers::core::types::Address,
        pub total_staked: ::ethers::core::types::U256,
        pub unstake_delay_sec: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "StakeUnlocked", abi = "StakeUnlocked(address,uint256)")]
    pub struct StakeUnlockedFilter {
        #[ethevent(indexed)]
        pub account: ::ethers::core::types::Address,
        pub withdraw_time: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(
        name = "StakeWithdrawn",
        abi = "StakeWithdrawn(address,address,uint256)"
    )]
    pub struct StakeWithdrawnFilter {
        #[ethevent(indexed)]
        pub account: ::ethers::core::types::Address,
        pub withdraw_address: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(
        name = "UserOperationEvent",
        abi = "UserOperationEvent(bytes32,address,address,uint256,bool,uint256,uint256)"
    )]
    pub struct UserOperationEventFilter {
        #[ethevent(indexed)]
        pub user_op_hash: [u8; 32],
        #[ethevent(indexed)]
        pub sender: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub paymaster: ::ethers::core::types::Address,
        pub nonce: ::ethers::core::types::U256,
        pub success: bool,
        pub actual_gas_cost: ::ethers::core::types::U256,
        pub actual_gas_used: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(
        name = "UserOperationRevertReason",
        abi = "UserOperationRevertReason(bytes32,address,uint256,bytes)"
    )]
    pub struct UserOperationRevertReasonFilter {
        #[ethevent(indexed)]
        pub user_op_hash: [u8; 32],
        #[ethevent(indexed)]
        pub sender: ::ethers::core::types::Address,
        pub nonce: ::ethers::core::types::U256,
        pub revert_reason: ::ethers::core::types::Bytes,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "Withdrawn", abi = "Withdrawn(address,address,uint256)")]
    pub struct WithdrawnFilter {
        #[ethevent(indexed)]
        pub account: ::ethers::core::types::Address,
        pub withdraw_address: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum EntryPointEvents {
        AccountDeployedFilter(AccountDeployedFilter),
        BeforeExecutionFilter(BeforeExecutionFilter),
        DepositedFilter(DepositedFilter),
        SignatureAggregatorChangedFilter(SignatureAggregatorChangedFilter),
        StakeLockedFilter(StakeLockedFilter),
        StakeUnlockedFilter(StakeUnlockedFilter),
        StakeWithdrawnFilter(StakeWithdrawnFilter),
        UserOperationEventFilter(UserOperationEventFilter),
        UserOperationRevertReasonFilter(UserOperationRevertReasonFilter),
        WithdrawnFilter(WithdrawnFilter),
    }
    impl ::ethers::contract::EthLogDecode for EntryPointEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = AccountDeployedFilter::decode_log(log) {
                return Ok(EntryPointEvents::AccountDeployedFilter(decoded));
            }
            if let Ok(decoded) = BeforeExecutionFilter::decode_log(log) {
                return Ok(EntryPointEvents::BeforeExecutionFilter(decoded));
            }
            if let Ok(decoded) = DepositedFilter::decode_log(log) {
                return Ok(EntryPointEvents::DepositedFilter(decoded));
            }
            if let Ok(decoded) = SignatureAggregatorChangedFilter::decode_log(log) {
                return Ok(EntryPointEvents::SignatureAggregatorChangedFilter(decoded));
            }
            if let Ok(decoded) = StakeLockedFilter::decode_log(log) {
                return Ok(EntryPointEvents::StakeLockedFilter(decoded));
            }
            if let Ok(decoded) = StakeUnlockedFilter::decode_log(log) {
                return Ok(EntryPointEvents::StakeUnlockedFilter(decoded));
            }
            if let Ok(decoded) = StakeWithdrawnFilter::decode_log(log) {
                return Ok(EntryPointEvents::StakeWithdrawnFilter(decoded));
            }
            if let Ok(decoded) = UserOperationEventFilter::decode_log(log) {
                return Ok(EntryPointEvents::UserOperationEventFilter(decoded));
            }
            if let Ok(decoded) = UserOperationRevertReasonFilter::decode_log(log) {
                return Ok(EntryPointEvents::UserOperationRevertReasonFilter(decoded));
            }
            if let Ok(decoded) = WithdrawnFilter::decode_log(log) {
                return Ok(EntryPointEvents::WithdrawnFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for EntryPointEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AccountDeployedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::BeforeExecutionFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::DepositedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::SignatureAggregatorChangedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::StakeLockedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::StakeUnlockedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::StakeWithdrawnFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::UserOperationEventFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::UserOperationRevertReasonFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::WithdrawnFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AccountDeployedFilter> for EntryPointEvents {
        fn from(value: AccountDeployedFilter) -> Self {
            Self::AccountDeployedFilter(value)
        }
    }
    impl ::core::convert::From<BeforeExecutionFilter> for EntryPointEvents {
        fn from(value: BeforeExecutionFilter) -> Self {
            Self::BeforeExecutionFilter(value)
        }
    }
    impl ::core::convert::From<DepositedFilter> for EntryPointEvents {
        fn from(value: DepositedFilter) -> Self {
            Self::DepositedFilter(value)
        }
    }
    impl ::core::convert::From<SignatureAggregatorChangedFilter> for EntryPointEvents {
        fn from(value: SignatureAggregatorChangedFilter) -> Self {
            Self::SignatureAggregatorChangedFilter(value)
        }
    }
    impl ::core::convert::From<StakeLockedFilter> for EntryPointEvents {
        fn from(value: StakeLockedFilter) -> Self {
            Self::StakeLockedFilter(value)
        }
    }
    impl ::core::convert::From<StakeUnlockedFilter> for EntryPointEvents {
        fn from(value: StakeUnlockedFilter) -> Self {
            Self::StakeUnlockedFilter(value)
        }
    }
    impl ::core::convert::From<StakeWithdrawnFilter> for EntryPointEvents {
        fn from(value: StakeWithdrawnFilter) -> Self {
            Self::StakeWithdrawnFilter(value)
        }
    }
    impl ::core::convert::From<UserOperationEventFilter> for EntryPointEvents {
        fn from(value: UserOperationEventFilter) -> Self {
            Self::UserOperationEventFilter(value)
        }
    }
    impl ::core::convert::From<UserOperationRevertReasonFilter> for EntryPointEvents {
        fn from(value: UserOperationRevertReasonFilter) -> Self {
            Self::UserOperationRevertReasonFilter(value)
        }
    }
    impl ::core::convert::From<WithdrawnFilter> for EntryPointEvents {
        fn from(value: WithdrawnFilter) -> Self {
            Self::WithdrawnFilter(value)
        }
    }
    ///Container type for all input parameters for the `SIG_VALIDATION_FAILED` function with signature `SIG_VALIDATION_FAILED()` and selector `0x8f41ec5a`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "SIG_VALIDATION_FAILED", abi = "SIG_VALIDATION_FAILED()")]
    pub struct SigValidationFailedCall;
    ///Container type for all input parameters for the `_validateSenderAndPaymaster` function with signature `_validateSenderAndPaymaster(bytes,address,bytes)` and selector `0x957122ab`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "_validateSenderAndPaymaster",
        abi = "_validateSenderAndPaymaster(bytes,address,bytes)"
    )]
    pub struct ValidateSenderAndPaymasterCall {
        pub init_code: ::ethers::core::types::Bytes,
        pub sender: ::ethers::core::types::Address,
        pub paymaster_and_data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `addStake` function with signature `addStake(uint32)` and selector `0x0396cb60`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "addStake", abi = "addStake(uint32)")]
    pub struct AddStakeCall {
        pub unstake_delay_sec: u32,
    }
    ///Container type for all input parameters for the `balanceOf` function with signature `balanceOf(address)` and selector `0x70a08231`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "balanceOf", abi = "balanceOf(address)")]
    pub struct BalanceOfCall {
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `depositTo` function with signature `depositTo(address)` and selector `0xb760faf9`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "depositTo", abi = "depositTo(address)")]
    pub struct DepositToCall {
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `deposits` function with signature `deposits(address)` and selector `0xfc7e286d`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "deposits", abi = "deposits(address)")]
    pub struct DepositsCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `getDepositInfo` function with signature `getDepositInfo(address)` and selector `0x5287ce12`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getDepositInfo", abi = "getDepositInfo(address)")]
    pub struct GetDepositInfoCall {
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getNonce` function with signature `getNonce(address,uint192)` and selector `0x35567e1a`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getNonce", abi = "getNonce(address,uint192)")]
    pub struct GetNonceCall {
        pub sender: ::ethers::core::types::Address,
        pub key: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getSenderAddress` function with signature `getSenderAddress(bytes)` and selector `0x9b249f69`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getSenderAddress", abi = "getSenderAddress(bytes)")]
    pub struct GetSenderAddressCall {
        pub init_code: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `getUserOpHash` function with signature `getUserOpHash((address,uint256,bytes,bytes,uint256,uint256,uint256,uint256,uint256,bytes,bytes))` and selector `0xa6193531`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "getUserOpHash",
        abi = "getUserOpHash((address,uint256,bytes,bytes,uint256,uint256,uint256,uint256,uint256,bytes,bytes))"
    )]
    pub struct GetUserOpHashCall {
        pub user_op: UserOperation,
    }
    ///Container type for all input parameters for the `handleAggregatedOps` function with signature `handleAggregatedOps(((address,uint256,bytes,bytes,uint256,uint256,uint256,uint256,uint256,bytes,bytes)[],address,bytes)[],address)` and selector `0x4b1d7cf5`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "handleAggregatedOps",
        abi = "handleAggregatedOps(((address,uint256,bytes,bytes,uint256,uint256,uint256,uint256,uint256,bytes,bytes)[],address,bytes)[],address)"
    )]
    pub struct HandleAggregatedOpsCall {
        pub ops_per_aggregator: ::std::vec::Vec<UserOpsPerAggregator>,
        pub beneficiary: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `handleOps` function with signature `handleOps((address,uint256,bytes,bytes,uint256,uint256,uint256,uint256,uint256,bytes,bytes)[],address)` and selector `0x1fad948c`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "handleOps",
        abi = "handleOps((address,uint256,bytes,bytes,uint256,uint256,uint256,uint256,uint256,bytes,bytes)[],address)"
    )]
    pub struct HandleOpsCall {
        pub ops: ::std::vec::Vec<UserOperation>,
        pub beneficiary: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `incrementNonce` function with signature `incrementNonce(uint192)` and selector `0x0bd28e3b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "incrementNonce", abi = "incrementNonce(uint192)")]
    pub struct IncrementNonceCall {
        pub key: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `innerHandleOp` function with signature `innerHandleOp(bytes,((address,uint256,uint256,uint256,uint256,address,uint256,uint256),bytes32,uint256,uint256,uint256),bytes)` and selector `0x1d732756`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "innerHandleOp",
        abi = "innerHandleOp(bytes,((address,uint256,uint256,uint256,uint256,address,uint256,uint256),bytes32,uint256,uint256,uint256),bytes)"
    )]
    pub struct InnerHandleOpCall {
        pub call_data: ::ethers::core::types::Bytes,
        pub op_info: UserOpInfo,
        pub context: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `nonceSequenceNumber` function with signature `nonceSequenceNumber(address,uint192)` and selector `0x1b2e01b8`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "nonceSequenceNumber",
        abi = "nonceSequenceNumber(address,uint192)"
    )]
    pub struct NonceSequenceNumberCall(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all input parameters for the `simulateHandleOp` function with signature `simulateHandleOp((address,uint256,bytes,bytes,uint256,uint256,uint256,uint256,uint256,bytes,bytes),address,bytes)` and selector `0xd6383f94`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "simulateHandleOp",
        abi = "simulateHandleOp((address,uint256,bytes,bytes,uint256,uint256,uint256,uint256,uint256,bytes,bytes),address,bytes)"
    )]
    pub struct SimulateHandleOpCall {
        pub op: UserOperation,
        pub target: ::ethers::core::types::Address,
        pub target_call_data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `simulateValidation` function with signature `simulateValidation((address,uint256,bytes,bytes,uint256,uint256,uint256,uint256,uint256,bytes,bytes))` and selector `0xee219423`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "simulateValidation",
        abi = "simulateValidation((address,uint256,bytes,bytes,uint256,uint256,uint256,uint256,uint256,bytes,bytes))"
    )]
    pub struct SimulateValidationCall {
        pub user_op: UserOperation,
    }
    ///Container type for all input parameters for the `unlockStake` function with signature `unlockStake()` and selector `0xbb9fe6bf`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "unlockStake", abi = "unlockStake()")]
    pub struct UnlockStakeCall;
    ///Container type for all input parameters for the `withdrawStake` function with signature `withdrawStake(address)` and selector `0xc23a5cea`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "withdrawStake", abi = "withdrawStake(address)")]
    pub struct WithdrawStakeCall {
        pub withdraw_address: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `withdrawTo` function with signature `withdrawTo(address,uint256)` and selector `0x205c2878`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "withdrawTo", abi = "withdrawTo(address,uint256)")]
    pub struct WithdrawToCall {
        pub withdraw_address: ::ethers::core::types::Address,
        pub withdraw_amount: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum EntryPointCalls {
        SigValidationFailed(SigValidationFailedCall),
        ValidateSenderAndPaymaster(ValidateSenderAndPaymasterCall),
        AddStake(AddStakeCall),
        BalanceOf(BalanceOfCall),
        DepositTo(DepositToCall),
        Deposits(DepositsCall),
        GetDepositInfo(GetDepositInfoCall),
        GetNonce(GetNonceCall),
        GetSenderAddress(GetSenderAddressCall),
        GetUserOpHash(GetUserOpHashCall),
        HandleAggregatedOps(HandleAggregatedOpsCall),
        HandleOps(HandleOpsCall),
        IncrementNonce(IncrementNonceCall),
        InnerHandleOp(InnerHandleOpCall),
        NonceSequenceNumber(NonceSequenceNumberCall),
        SimulateHandleOp(SimulateHandleOpCall),
        SimulateValidation(SimulateValidationCall),
        UnlockStake(UnlockStakeCall),
        WithdrawStake(WithdrawStakeCall),
        WithdrawTo(WithdrawToCall),
    }
    impl ::ethers::core::abi::AbiDecode for EntryPointCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) =
                <SigValidationFailedCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SigValidationFailed(decoded));
            }
            if let Ok(decoded) =
                <ValidateSenderAndPaymasterCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ValidateSenderAndPaymaster(decoded));
            }
            if let Ok(decoded) = <AddStakeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AddStake(decoded));
            }
            if let Ok(decoded) = <BalanceOfCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::BalanceOf(decoded));
            }
            if let Ok(decoded) = <DepositToCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DepositTo(decoded));
            }
            if let Ok(decoded) = <DepositsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Deposits(decoded));
            }
            if let Ok(decoded) =
                <GetDepositInfoCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetDepositInfo(decoded));
            }
            if let Ok(decoded) = <GetNonceCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetNonce(decoded));
            }
            if let Ok(decoded) =
                <GetSenderAddressCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetSenderAddress(decoded));
            }
            if let Ok(decoded) = <GetUserOpHashCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetUserOpHash(decoded));
            }
            if let Ok(decoded) =
                <HandleAggregatedOpsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::HandleAggregatedOps(decoded));
            }
            if let Ok(decoded) = <HandleOpsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::HandleOps(decoded));
            }
            if let Ok(decoded) =
                <IncrementNonceCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::IncrementNonce(decoded));
            }
            if let Ok(decoded) = <InnerHandleOpCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::InnerHandleOp(decoded));
            }
            if let Ok(decoded) =
                <NonceSequenceNumberCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::NonceSequenceNumber(decoded));
            }
            if let Ok(decoded) =
                <SimulateHandleOpCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SimulateHandleOp(decoded));
            }
            if let Ok(decoded) =
                <SimulateValidationCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SimulateValidation(decoded));
            }
            if let Ok(decoded) = <UnlockStakeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::UnlockStake(decoded));
            }
            if let Ok(decoded) = <WithdrawStakeCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::WithdrawStake(decoded));
            }
            if let Ok(decoded) = <WithdrawToCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::WithdrawTo(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for EntryPointCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::SigValidationFailed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ValidateSenderAndPaymaster(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AddStake(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::BalanceOf(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::DepositTo(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Deposits(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetDepositInfo(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetNonce(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetSenderAddress(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetUserOpHash(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::HandleAggregatedOps(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::HandleOps(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::IncrementNonce(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InnerHandleOp(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::NonceSequenceNumber(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SimulateHandleOp(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SimulateValidation(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UnlockStake(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::WithdrawStake(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::WithdrawTo(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for EntryPointCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::SigValidationFailed(element) => ::core::fmt::Display::fmt(element, f),
                Self::ValidateSenderAndPaymaster(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddStake(element) => ::core::fmt::Display::fmt(element, f),
                Self::BalanceOf(element) => ::core::fmt::Display::fmt(element, f),
                Self::DepositTo(element) => ::core::fmt::Display::fmt(element, f),
                Self::Deposits(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetDepositInfo(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetNonce(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetSenderAddress(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetUserOpHash(element) => ::core::fmt::Display::fmt(element, f),
                Self::HandleAggregatedOps(element) => ::core::fmt::Display::fmt(element, f),
                Self::HandleOps(element) => ::core::fmt::Display::fmt(element, f),
                Self::IncrementNonce(element) => ::core::fmt::Display::fmt(element, f),
                Self::InnerHandleOp(element) => ::core::fmt::Display::fmt(element, f),
                Self::NonceSequenceNumber(element) => ::core::fmt::Display::fmt(element, f),
                Self::SimulateHandleOp(element) => ::core::fmt::Display::fmt(element, f),
                Self::SimulateValidation(element) => ::core::fmt::Display::fmt(element, f),
                Self::UnlockStake(element) => ::core::fmt::Display::fmt(element, f),
                Self::WithdrawStake(element) => ::core::fmt::Display::fmt(element, f),
                Self::WithdrawTo(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<SigValidationFailedCall> for EntryPointCalls {
        fn from(value: SigValidationFailedCall) -> Self {
            Self::SigValidationFailed(value)
        }
    }
    impl ::core::convert::From<ValidateSenderAndPaymasterCall> for EntryPointCalls {
        fn from(value: ValidateSenderAndPaymasterCall) -> Self {
            Self::ValidateSenderAndPaymaster(value)
        }
    }
    impl ::core::convert::From<AddStakeCall> for EntryPointCalls {
        fn from(value: AddStakeCall) -> Self {
            Self::AddStake(value)
        }
    }
    impl ::core::convert::From<BalanceOfCall> for EntryPointCalls {
        fn from(value: BalanceOfCall) -> Self {
            Self::BalanceOf(value)
        }
    }
    impl ::core::convert::From<DepositToCall> for EntryPointCalls {
        fn from(value: DepositToCall) -> Self {
            Self::DepositTo(value)
        }
    }
    impl ::core::convert::From<DepositsCall> for EntryPointCalls {
        fn from(value: DepositsCall) -> Self {
            Self::Deposits(value)
        }
    }
    impl ::core::convert::From<GetDepositInfoCall> for EntryPointCalls {
        fn from(value: GetDepositInfoCall) -> Self {
            Self::GetDepositInfo(value)
        }
    }
    impl ::core::convert::From<GetNonceCall> for EntryPointCalls {
        fn from(value: GetNonceCall) -> Self {
            Self::GetNonce(value)
        }
    }
    impl ::core::convert::From<GetSenderAddressCall> for EntryPointCalls {
        fn from(value: GetSenderAddressCall) -> Self {
            Self::GetSenderAddress(value)
        }
    }
    impl ::core::convert::From<GetUserOpHashCall> for EntryPointCalls {
        fn from(value: GetUserOpHashCall) -> Self {
            Self::GetUserOpHash(value)
        }
    }
    impl ::core::convert::From<HandleAggregatedOpsCall> for EntryPointCalls {
        fn from(value: HandleAggregatedOpsCall) -> Self {
            Self::HandleAggregatedOps(value)
        }
    }
    impl ::core::convert::From<HandleOpsCall> for EntryPointCalls {
        fn from(value: HandleOpsCall) -> Self {
            Self::HandleOps(value)
        }
    }
    impl ::core::convert::From<IncrementNonceCall> for EntryPointCalls {
        fn from(value: IncrementNonceCall) -> Self {
            Self::IncrementNonce(value)
        }
    }
    impl ::core::convert::From<InnerHandleOpCall> for EntryPointCalls {
        fn from(value: InnerHandleOpCall) -> Self {
            Self::InnerHandleOp(value)
        }
    }
    impl ::core::convert::From<NonceSequenceNumberCall> for EntryPointCalls {
        fn from(value: NonceSequenceNumberCall) -> Self {
            Self::NonceSequenceNumber(value)
        }
    }
    impl ::core::convert::From<SimulateHandleOpCall> for EntryPointCalls {
        fn from(value: SimulateHandleOpCall) -> Self {
            Self::SimulateHandleOp(value)
        }
    }
    impl ::core::convert::From<SimulateValidationCall> for EntryPointCalls {
        fn from(value: SimulateValidationCall) -> Self {
            Self::SimulateValidation(value)
        }
    }
    impl ::core::convert::From<UnlockStakeCall> for EntryPointCalls {
        fn from(value: UnlockStakeCall) -> Self {
            Self::UnlockStake(value)
        }
    }
    impl ::core::convert::From<WithdrawStakeCall> for EntryPointCalls {
        fn from(value: WithdrawStakeCall) -> Self {
            Self::WithdrawStake(value)
        }
    }
    impl ::core::convert::From<WithdrawToCall> for EntryPointCalls {
        fn from(value: WithdrawToCall) -> Self {
            Self::WithdrawTo(value)
        }
    }
    ///Container type for all return fields from the `SIG_VALIDATION_FAILED` function with signature `SIG_VALIDATION_FAILED()` and selector `0x8f41ec5a`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct SigValidationFailedReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `balanceOf` function with signature `balanceOf(address)` and selector `0x70a08231`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct BalanceOfReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `deposits` function with signature `deposits(address)` and selector `0xfc7e286d`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct DepositsReturn {
        pub deposit: u128,
        pub staked: bool,
        pub stake: u128,
        pub unstake_delay_sec: u32,
        pub withdraw_time: u64,
    }
    ///Container type for all return fields from the `getDepositInfo` function with signature `getDepositInfo(address)` and selector `0x5287ce12`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetDepositInfoReturn {
        pub info: DepositInfo,
    }
    ///Container type for all return fields from the `getNonce` function with signature `getNonce(address,uint192)` and selector `0x35567e1a`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetNonceReturn {
        pub nonce: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `getUserOpHash` function with signature `getUserOpHash((address,uint256,bytes,bytes,uint256,uint256,uint256,uint256,uint256,bytes,bytes))` and selector `0xa6193531`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetUserOpHashReturn(pub [u8; 32]);
    ///Container type for all return fields from the `innerHandleOp` function with signature `innerHandleOp(bytes,((address,uint256,uint256,uint256,uint256,address,uint256,uint256),bytes32,uint256,uint256,uint256),bytes)` and selector `0x1d732756`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct InnerHandleOpReturn {
        pub actual_gas_cost: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `nonceSequenceNumber` function with signature `nonceSequenceNumber(address,uint192)` and selector `0x1b2e01b8`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct NonceSequenceNumberReturn(pub ::ethers::core::types::U256);
    ///`MemoryUserOp(address,uint256,uint256,uint256,uint256,address,uint256,uint256)`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct MemoryUserOp {
        pub sender: ::ethers::core::types::Address,
        pub nonce: ::ethers::core::types::U256,
        pub call_gas_limit: ::ethers::core::types::U256,
        pub verification_gas_limit: ::ethers::core::types::U256,
        pub pre_verification_gas: ::ethers::core::types::U256,
        pub paymaster: ::ethers::core::types::Address,
        pub max_fee_per_gas: ::ethers::core::types::U256,
        pub max_priority_fee_per_gas: ::ethers::core::types::U256,
    }
    ///`UserOpInfo((address,uint256,uint256,uint256,uint256,address,uint256,uint256),bytes32,uint256,uint256,uint256)`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct UserOpInfo {
        pub m_user_op: MemoryUserOp,
        pub user_op_hash: [u8; 32],
        pub prefund: ::ethers::core::types::U256,
        pub context_offset: ::ethers::core::types::U256,
        pub pre_op_gas: ::ethers::core::types::U256,
    }
    ///`UserOpsPerAggregator((address,uint256,bytes,bytes,uint256,uint256,uint256,uint256,uint256,bytes,bytes)[],address,bytes)`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct UserOpsPerAggregator {
        pub user_ops: ::std::vec::Vec<UserOperation>,
        pub aggregator: ::ethers::core::types::Address,
        pub signature: ::ethers::core::types::Bytes,
    }
    ///`DepositInfo(uint112,bool,uint112,uint32,uint48)`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct DepositInfo {
        pub deposit: u128,
        pub staked: bool,
        pub stake: u128,
        pub unstake_delay_sec: u32,
        pub withdraw_time: u64,
    }
    ///`UserOperation(address,uint256,bytes,bytes,uint256,uint256,uint256,uint256,uint256,bytes,bytes)`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct UserOperation {
        pub sender: ::ethers::core::types::Address,
        pub nonce: ::ethers::core::types::U256,
        pub init_code: ::ethers::core::types::Bytes,
        pub call_data: ::ethers::core::types::Bytes,
        pub call_gas_limit: ::ethers::core::types::U256,
        pub verification_gas_limit: ::ethers::core::types::U256,
        pub pre_verification_gas: ::ethers::core::types::U256,
        pub max_fee_per_gas: ::ethers::core::types::U256,
        pub max_priority_fee_per_gas: ::ethers::core::types::U256,
        pub paymaster_and_data: ::ethers::core::types::Bytes,
        pub signature: ::ethers::core::types::Bytes,
    }
}
