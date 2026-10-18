/* Memory map for MPS2-AN505, using Non-Secure memory */
MEMORY
{
  /* NOTE K = KiBi = 1024 bytes */
  FLASH : ORIGIN = 0x10000000, LENGTH = 4M
  RAM : ORIGIN = 0x28000000, LENGTH = 2M
}
