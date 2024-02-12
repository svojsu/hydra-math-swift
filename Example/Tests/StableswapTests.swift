import XCTest
import HydraMath

class StableswapTests: XCTestCase {
    func testCalculateOutGiveInShouldWorkWhenCorrectJsonFormatProvided() {
        let data = """
        [{
            "asset_id": 1,
            "amount": "1000000000000",
            "decimals": 12
        },
        {
            "asset_id": 0,
            "amount": "1000000000000",
            "decimals": 12
        }
        ]
        """
        
        let result = HydraStableswapMath.calculateOutGivenIn(
            data,
            0,
            1,
            "1000000000",
            "1",
            "0"
        )
        
        XCTAssertEqual(result.toString(), "999500248")
    }
    
    func testCalculateSharesShouldWorkWhenCorrectJsonFormatProvided() {
        let data = """
        [{
            "asset_id": 0,
            "amount":"90000000000",
            "decimals": 12
        },
        {
            "asset_id": 1,
            "amount": "5000000000000000000000",
            "decimals": 12
        }
        ]
        """
        
        let assets = """
        [{"asset_id":1,"amount":"43000000000000000000"}]
        """
        
        let result = HydraStableswapMath.calculateShares(
            data,
            assets,
            "1000",
            "64839594451719860",
            "0"
        )
        
        XCTAssertEqual(result.toString(), "371541351762585")
    }
    
    func testPerformanceExample() {
        // This is an example of a performance test case.
        self.measure() {
            // Put the code you want to measure the time of here.
        }
    }
    
}
