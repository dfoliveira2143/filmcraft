# FilmCraft — funcionalidades e módulos

Última atualização: 2026-10-28.

## Fluxo principal

| Etapa | Entradas | Responsabilidade e dependências | Saídas |
|---|---|---|---|
| Abrir e editar | Arquivos de mídia, projeto e operações na timeline | [media](../../crates/media/), [project](../../crates/project/), [edit](../../crates/edit/), [engine](../../crates/engine/) | Sequência com clips, trilhas e tempo exato |
| Avaliar e produzir saída | Estado editado e parâmetros de saída | [render](../../crates/render/), [gpu](../../crates/gpu/), [export](../../crates/export/), [codecs](../../crates/codecs/) | Frames de preview e mídia exportada |
| Acesso pelo usuário ou agente | Gestos, argumentos CLI ou chamadas estruturadas | [apps](../../apps/), [engine](../../crates/engine/) e interfaces descritas abaixo | Estado atualizado, resposta ou erro; comportamento detalhado nos contratos |

A tabela mostra o caminho conceitual, não uma garantia de suporte a todos os formatos. APIs, estruturas, unidades, efeitos colaterais e limites ficam na [arquitetura](../architecture.md), nos documentos de assunto e nos módulos indicados.

## Contratos e exemplos de uso

- [project-files.md](../project-files.md)
- [captions.md](../captions.md)
- [transcripts.md](../transcripts.md)
- [codec-parity.md](../codec-parity.md)
- [testing.md](../testing.md)
- [agents.md](../agents.md)
- [control-protocol.md](../control-protocol.md)

## Inventário de bibliotecas

Responsabilidades resumidas a partir dos manifestos e comentários de módulo do checkout. As dependências listadas são as internas diretas declaradas em `dependencies` (incluindo condicionais); não incluem dependências de teste/build nem expansão de features. O manifesto de cada linha é a fonte exata.

| Módulo / manifesto | Responsabilidade declarada | Dependências internas diretas |
|---|---|---|
| [aac](../../crates/aac/Cargo.toml) | Clean-room AAC-LC encoder and decoder (ISO/IEC 14496-3) | `filmcraft-bitstream` |
| [ac3](../../crates/ac3/Cargo.toml) | Clean-room AC-3 and E-AC-3 (Dolby Digital / Dolby Digital Plus, ATSC A/52) audio decoder | Nenhuma declarada nesta categoria |
| [apv](../../crates/apv/Cargo.toml) | Clean-room Advanced Professional Video (APV, IETF RFC 9924) decoder and encoder | `filmcraft-bitstream` |
| [audio-dsp](../../crates/audio-dsp/Cargo.toml) | Real-time audio DSP for FilmCraft: BS.1770 / EBU R128 loudness metering and audio effects | Nenhuma declarada nesta categoria |
| [automation](../../crates/automation/Cargo.toml) | MCP server (rmcp) over the FilmCraft engine, plus a bridge to the running app's control channel | `filmcraft-engine`, `filmcraft-time` |
| [av1](../../crates/av1/Cargo.toml) | Clean-room AV1 decoder (AV1 Bitstream & Decoding Process Specification v1.0.0 with Errata 1) | `filmcraft-bitstream` |
| [avi](../../crates/avi/Cargo.toml) | Clean-room AVI (RIFF) and OpenDML demuxer | Nenhuma declarada nesta categoria |
| [bitstream](../../crates/bitstream/Cargo.toml) | Bit-level reading and writing: a re-export of `craft_media::bitstream` from craft-libs. | Nenhuma declarada nesta categoria |
| [captions](../../crates/captions/Cargo.toml) | FilmCraft captions: SRT, WebVTT, SCC, MCC, EBU STL and TTML/IMSC1/DFXP import/export, caption layout and burn-in rasterisation | `filmcraft-color`, `filmcraft-project`, `filmcraft-text`, `filmcraft-time` |
| [cfb](../../crates/cfb/Cargo.toml) | Clean-room Compound File Binary (Microsoft structured storage, [MS-CFB]) reader and writer: the container of AAF files | Nenhuma declarada nesta categoria |
| [codecs](../../crates/codecs/Cargo.toml) | Container + codec hub: MP4/MOV, Matroska, MXF, Ogg and MPEG TS/PS sources, image sequences, GOP-aware seeking, decoder registry, audio decoding | `filmcraft-aac`, `filmcraft-ac3`, `filmcraft-apv`, `filmcraft-av1`, `filmcraft-avi`, `filmcraft-bitstream`, `filmcraft-color`, `filmcraft-dnx`, `filmcraft-frame`, `filmcraft-h264`, `filmcraft-hevc`, `filmcraft-isobmff`, `filmcraft-matroska`, `filmcraft-media`, `filmcraft-mpa`, `filmcraft-mpeg2v`, `filmcraft-mpegts`, `filmcraft-mxf`, `filmcraft-ogg`, `filmcraft-opus`, `filmcraft-prores`, `filmcraft-time`, `filmcraft-vp8`, `filmcraft-vp9` |
| [color](../../crates/color/Cargo.toml) | Colour science for FilmCraft: YUV matrices, transfer functions, camera log curves, gamuts, colour-managed input/output transforms (tone and gamut mapping), colour metadata and LUTs. | Nenhuma declarada nesta categoria |
| [dng](../../crates/dng/Cargo.toml) | Clean-room DNG / CinemaDNG camera RAW decoder (TIFF container, lossless JPEG, DNG colour pipeline) | Nenhuma declarada nesta categoria |
| [dnx](../../crates/dnx/Cargo.toml) | Clean-room Avid DNxHD / DNxHR (SMPTE ST 2019-1 VC-3) decoder and encoder | `filmcraft-bitstream` |
| [edit](../../crates/edit/Cargo.toml) | Pure timeline edit algebra: insert, overwrite, razor, ripple, roll, slip, slide, rate stretch, lift, extract | `filmcraft-project`, `filmcraft-time` |
| [engine](../../crates/engine/Cargo.toml) | FilmCraft engine façade: session, command registry, history, media pool | `filmcraft-audio-dsp`, `filmcraft-captions`, `filmcraft-codecs`, `filmcraft-color`, `filmcraft-edit`, `filmcraft-export`, `filmcraft-format`, `filmcraft-frame`, `filmcraft-geom`, `filmcraft-interchange`, `filmcraft-media`, `filmcraft-mxf`, `filmcraft-project`, `filmcraft-render`, `filmcraft-scopes`, `filmcraft-speech`, `filmcraft-text`, `filmcraft-time`, `filmcraft-tts` |
| [export](../../crates/export/Cargo.toml) | Export pipeline: parallel render → encode → mux, presets, progress/cancel | `filmcraft-aac`, `filmcraft-apv`, `filmcraft-audio-dsp`, `filmcraft-color`, `filmcraft-dnx`, `filmcraft-flac`, `filmcraft-frame`, `filmcraft-geom`, `filmcraft-h264enc`, `filmcraft-isobmff`, `filmcraft-media`, `filmcraft-mxf`, `filmcraft-project`, `filmcraft-prores`, `filmcraft-render`, `filmcraft-text`, `filmcraft-time` |
| [flac](../../crates/flac/Cargo.toml) | Clean-room FLAC encoder (RFC 9639) | `filmcraft-bitstream` |
| [format](../../crates/format/Cargo.toml) | FilmCraft .fcproj files: schema versions + migrations, atomic saves, auto-save naming and rotation | `filmcraft-media`, `filmcraft-project` |
| [frame](../../crates/frame/Cargo.toml) | Video frames and audio buffers with explicit colour metadata | `filmcraft-color`, `filmcraft-time` |
| [geom](../../crates/geom/Cargo.toml) | 2D geometry for FilmCraft: vectors, rectangles and affine transforms (Motion effect math). | Nenhuma declarada nesta categoria |
| [golden](../../crates/golden/Cargo.toml) | Golden-image tests for the CPU renderer/compositor and GPU-vs-CPU parity (test-only crate) | Nenhuma declarada nesta categoria |
| [gpu](../../crates/gpu/Cargo.toml) | wgpu compositor: YUV/RGB layer sampling, linear-light premultiplied compositing in WGSL | `filmcraft-color`, `filmcraft-frame`, `filmcraft-geom`, `filmcraft-project`, `filmcraft-render`, `filmcraft-time` |
| [h264](../../crates/h264/Cargo.toml) | Clean-room pure-Rust H.264/AVC decoder (ITU-T H.264) | `filmcraft-bitstream` |
| [h264enc](../../crates/h264enc/Cargo.toml) | Clean-room pure-Rust H.264/AVC encoder (High/Main/Baseline, CABAC/CAVLC, B-frames, rate control) | `filmcraft-bitstream` |
| [hevc](../../crates/hevc/Cargo.toml) | Clean-room pure-Rust H.265/HEVC decoder (ITU-T H.265) | `filmcraft-bitstream` |
| [interchange](../../crates/interchange/Cargo.toml) | FilmCraft interchange: CMX 3600 EDL, Final Cut Pro 7 XML (xmeml), FCPXML and OpenTimelineIO import/export | `filmcraft-cfb`, `filmcraft-color`, `filmcraft-geom`, `filmcraft-media`, `filmcraft-project`, `filmcraft-time` |
| [isobmff](../../crates/isobmff/Cargo.toml) | Clean-room ISO BMFF (MP4) and QuickTime (MOV) demuxer and muxer | Nenhuma declarada nesta categoria |
| [matroska](../../crates/matroska/Cargo.toml) | Clean-room Matroska (MKV) and WebM demuxer and muxer | Nenhuma declarada nesta categoria |
| [media](../../crates/media/Cargo.toml) | Media sources: probing, decoding traits, frame cache, generators, stills | `filmcraft-color`, `filmcraft-dng`, `filmcraft-frame`, `filmcraft-time` |
| [mpa](../../crates/mpa/Cargo.toml) | Clean-room MPEG-1/2 Audio Layer I and Layer II (MP1 / MP2) decoder | `filmcraft-bitstream` |
| [mpeg2v](../../crates/mpeg2v/Cargo.toml) | Clean-room MPEG-2 video (ITU-T H.262 / ISO/IEC 13818-2) and MPEG-1 video (ISO/IEC 11172-2) decoder | Nenhuma declarada nesta categoria |
| [mpegts](../../crates/mpegts/Cargo.toml) | Clean-room MPEG-2 Systems (ISO/IEC 13818-1) demuxer: Transport Stream (188-byte, 192-byte M2TS) and Program Stream (MPEG-1/2 PS, VOB, MOD) | Nenhuma declarada nesta categoria |
| [mxf](../../crates/mxf/Cargo.toml) | Clean-room MXF (SMPTE ST 377-1) demuxer and writer: OP1a and OP-Atom, index tables, AVC/VC-3/ProRes/MPEG-2/PCM essence | Nenhuma declarada nesta categoria |
| [ogg](../../crates/ogg/Cargo.toml) | Clean-room Ogg (RFC 3533) demuxer with Ogg Opus (RFC 7845) and Ogg Vorbis stream mapping | Nenhuma declarada nesta categoria |
| [opus](../../crates/opus/Cargo.toml) | Clean-room pure-Rust Opus decoder (RFC 6716 / RFC 8251; OpusHead, multistream surround) | Nenhuma declarada nesta categoria |
| [platform](../../crates/platform/Cargo.toml) | OS media integration: hardware video decoding through the operating system's codecs (VideoToolbox on macOS, Media Foundation on Windows, VA-API and NVDEC on Linux), with transparent fallback to the pure-Rust decoders | `filmcraft-bitstream`, `filmcraft-codecs`, `filmcraft-color`, `filmcraft-export`, `filmcraft-frame`, `filmcraft-h264`, `filmcraft-hevc`, `filmcraft-isobmff`, `filmcraft-time` |
| [project](../../crates/project/Cargo.toml) | FilmCraft document model: projects, bins, clips, sequences, tracks, effects, keyframes, markers | `filmcraft-color`, `filmcraft-geom`, `filmcraft-media`, `filmcraft-time` |
| [prores](../../crates/prores/Cargo.toml) | Clean-room Apple ProRes (SMPTE RDD 36) decoder and encoder | `filmcraft-bitstream` |
| [render](../../crates/render/Cargo.toml) | Sequence evaluation and CPU reference compositor (linear-light premultiplied f32) | `filmcraft-audio-dsp`, `filmcraft-captions`, `filmcraft-color`, `filmcraft-frame`, `filmcraft-geom`, `filmcraft-media`, `filmcraft-project`, `filmcraft-text`, `filmcraft-time` |
| [scopes](../../crates/scopes/Cargo.toml) | Video scopes maths: waveform, parade, histogram and vectorscopes from a frame's R'G'B' values | `filmcraft-color` |
| [speech](../../crates/speech/Cargo.toml) | FilmCraft speech-to-text: the Transcriber trait, the Whisper model catalogue and downloader, pure-Rust Whisper inference (candle) with word timestamps, and speaker labelling | `filmcraft-project`, `filmcraft-time` |
| [testkit](../../crates/testkit/Cargo.toml) | Test-only helpers: ffmpeg/ffprobe oracle discovery, fixture directories, golden-image comparison | Nenhuma declarada nesta categoria |
| [text](../../crates/text/Cargo.toml) | FilmCraft text engine: font database (bundled + system), shaping, bidi, line breaking, paragraph layout and anti-aliased glyph/path rasterisation | `filmcraft-color` |
| [time](../../crates/time/Cargo.toml) | Exact media time for FilmCraft. | Nenhuma declarada nesta categoria |
| [tts](../../crates/tts/Cargo.toml) | FilmCraft text to speech: the Voice trait, the voice catalogue, script parsing (pause markers), the built-in formant voice and (feature kokoro) Kokoro-82M neural voices on candle | `filmcraft-audio-dsp`, `filmcraft-tts-text` |
| [tts-text](../../crates/tts-text/Cargo.toml) | FilmCraft text-to-speech text front end: text normalization (numbers, money, dates, times, abbreviations, acronyms, pause markers) into plain spoken words, English and Spanish | Nenhuma declarada nesta categoria |
| [ui-egui](../../crates/ui-egui/Cargo.toml) | The FilmCraft egui frontend (swappable): panels, docking, timeline, monitors, control channel | `filmcraft-audio-dsp`, `filmcraft-color`, `filmcraft-edit`, `filmcraft-engine`, `filmcraft-export`, `filmcraft-frame`, `filmcraft-geom`, `filmcraft-gpu`, `filmcraft-media`, `filmcraft-project`, `filmcraft-render`, `filmcraft-scopes`, `filmcraft-text`, `filmcraft-time`, `filmcraft-tts` |
| [vp8](../../crates/vp8/Cargo.toml) | Clean-room pure-Rust VP8 decoder (RFC 6386) | Nenhuma declarada nesta categoria |
| [vp9](../../crates/vp9/Cargo.toml) | Clean-room pure-Rust VP9 decoder (VP9 Bitstream & Decoding Process Specification) | `filmcraft-bitstream` |

Ao mudar uma entrada, saída ou dependência, atualize primeiro o contrato do módulo e depois esta navegação. [Voltar ao índice](../INDEX.md).
