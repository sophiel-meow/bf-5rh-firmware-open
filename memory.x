/* AT32F421C8T7: 64K flash total, split by the stock KDH_Bootloader into
 * a 4K bootloader region (0x08000000-0x08001000, left untouched) and a
 * 60K application region starting at 0x080010000
 */
MEMORY
{
  FLASH : ORIGIN = 0x08001000, LENGTH = 60K
  RAM   : ORIGIN = 0x20000000, LENGTH = 16K
}
