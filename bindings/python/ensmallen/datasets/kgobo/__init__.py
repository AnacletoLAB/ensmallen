"""This sub-module offers methods to automatically retrieve the graphs from KGOBO repository."""

from .mod import MOD
from .fbbt import FBBT
from .bto import BTO
from .chmo import CHMO
from .oba import OBA
from .pso import PSO
from .ogsf import OGSF
from .mco import MCO
from .opmi import OPMI
from .fbdv import FBDV
from .ceph import CEPH
from .mpath import MPATH
from .spd import SPD
from .omit import OMIT
from .vt import VT
from .ehdaa2 import EHDAA2
from .flopo import FLOPO
from .wbls import WBLS
from .rxno import RXNO
from .omp import OMP
from .ero import ERO
from .gno import GNO
from .xco import XCO
from .amphx import AMPHX
from .clyh import CLYH
from .oostt import OOSTT
from .fypo import FYPO
from .ncro import NCRO
from .iao import IAO
from .geo import GEO
from .exo import EXO
from .swo import SWO
from .obcs import OBCS
from .envo import ENVO
from .symp import SYMP
from .taxrank import TAXRANK
from .apo import APO
from .clo import CLO
from .cmo import CMO
from .ohmi import OHMI
from .hso import HSO
from .fbbi import FBBI
from .obi import OBI
from .cdao import CDAO
from .mfmo import MFMO
from .cro import CRO
from .cheminf import CHEMINF
from .mp import MP
from .duo import DUO
from .labo import LABO
from .olatdv import OLATDV
from .mpio import MPIO
from .chebi import CHEBI
from .aeo import AEO
from .geno import GENO
from .sbo import SBO
from .to import TO
from .uo import UO
from .mop import MOP
from .chiro import CHIRO
from .ogms import OGMS
from .ncbitaxon import NCBITAXON
from .foodon import FOODON
from .pw import PW
from .fovt import FOVT
from .xpo import XPO
from .zfs import ZFS
from .rs import RS
from .cto import CTO
from .omo import OMO
from .fix import FIX
from .mamo import MAMO
from .vto import VTO
from .uberon import UBERON
from .mfomd import MFOMD
from .bfo import BFO
from .htn import HTN
from .poro import PORO
from .aism import AISM
from .wbbt import WBBT
from .hao import HAO
from .so import SO
from .ro import RO
from .mondo import MONDO
from .ddpheno import DDPHENO
from .idomal import IDOMAL
from .maxo import MAXO
from .fbcv import FBCV
from .trans import TRANS
from .psdo import PSDO
from .scdo import SCDO
from .ontoneo import ONTONEO
from .dron import DRON
from .rbo import RBO
from .ncit import NCIT
from .fma import FMA
from .rex import REX
from .cob import COB
from .sibo import SIBO
from .pdro import PDRO
from .ogg import OGG
from .xlmod import XLMOD
from .hancestro import HANCESTRO
from .go import GO
from .mf import MF
from .gsso import GSSO
from .upheno import UPHENO
from .plana import PLANA
from .oae import OAE
from .mmusdv import MMUSDV
from .ms import MS
from .apollo_sv import APOLLO_SV
from .hsapdv import HSAPDV
from .vo import VO
from .miro import MIRO
from .emapa import EMAPA
from .gecko import GECKO
from .genepio import GENEPIO
from .tads import TADS
from .fao import FAO
from .cvdo import CVDO
from .ecao import ECAO
from .ohpi import OHPI
from .opl import OPL
from .tgma import TGMA
from .bco import BCO
from .ico import ICO
from .zeco import ZECO
from .phipo import PHIPO
from .pdumdv import PDUMDV
from .aro import ARO
from .oarcs import OARCS
from .cteno import CTENO
from .planp import PLANP
from .doid import DOID
from .omrse import OMRSE
from .ppo import PPO
from .ovae import OVAE
from .zp import ZP
from .stato import STATO
from .one import ONE
from .ecto import ECTO
from .xao import XAO
from .miapa import MIAPA
from .mi import MI
from .ecocore import ECOCORE
from .mmo import MMO
from .eupath import EUPATH
from .obib import OBIB
from .ido import IDO
from .sepio import SEPIO
from .tto import TTO
from .pr import PR
from .nbo import NBO
from .wbphenotype import WBPHENOTYPE
from .peco import PECO
from .gaz import GAZ
from .cio import CIO
from .ino import INO
from .clao import CLAO
from .upa import UPA
from .nomen import NOMEN
from .zfa import ZFA
from .disdriv import DISDRIV
from .cido import CIDO
from .kisao import KISAO
from .ma import MA
from .po import PO
from .cdno import CDNO
from .ohd import OHD
from .vario import VARIO
from .agro import AGRO
from .dideo import DIDEO
from .txpo import TXPO
from .pato import PATO
from .hom import HOM
from .eco import ECO
from .iceo import ICEO
from .ddanat import DDANAT
from .bspo import BSPO
from .mro import MRO
from .pco import PCO
from .epso import EPSO
from .ornaseq import ORNASEQ
from .hp import HP
from .dpo import DPO
from .cl import CL
from .mfoem import MFOEM

__all__ = [
	"MOD", "FBBT", "BTO", "CHMO", "OBA", "PSO", "OGSF", "MCO", "OPMI", "FBDV",
	"CEPH", "MPATH", "SPD", "OMIT", "VT", "EHDAA2", "FLOPO", "WBLS", "RXNO",
	"OMP", "ERO", "GNO", "XCO", "AMPHX", "CLYH", "OOSTT", "FYPO", "NCRO", "IAO",
	"GEO", "EXO", "SWO", "OBCS", "ENVO", "SYMP", "TAXRANK", "APO", "CLO", "CMO",
	"OHMI", "HSO", "FBBI", "OBI", "CDAO", "MFMO", "CRO", "CHEMINF", "MP", "DUO",
	"LABO", "OLATDV", "MPIO", "CHEBI", "AEO", "GENO", "SBO", "TO", "UO", "MOP",
	"CHIRO", "OGMS", "NCBITAXON", "FOODON", "PW", "FOVT", "XPO", "ZFS", "RS",
	"CTO", "OMO", "FIX", "MAMO", "VTO", "UBERON", "MFOMD", "BFO", "HTN", "PORO",
	"AISM", "WBBT", "HAO", "SO", "RO", "MONDO", "DDPHENO", "IDOMAL", "MAXO",
	"FBCV", "TRANS", "PSDO", "SCDO", "ONTONEO", "DRON", "RBO", "NCIT", "FMA",
	"REX", "COB", "SIBO", "PDRO", "OGG", "XLMOD", "HANCESTRO", "GO", "MF",
	"GSSO", "UPHENO", "PLANA", "OAE", "MMUSDV", "MS", "APOLLO_SV", "HSAPDV",
	"VO", "MIRO", "EMAPA", "GECKO", "GENEPIO", "TADS", "FAO", "CVDO", "ECAO",
	"OHPI", "OPL", "TGMA", "BCO", "ICO", "ZECO", "PHIPO", "PDUMDV", "ARO",
	"OARCS", "CTENO", "PLANP", "DOID", "OMRSE", "PPO", "OVAE", "ZP", "STATO",
	"ONE", "ECTO", "XAO", "MIAPA", "MI", "ECOCORE", "MMO", "EUPATH", "OBIB",
	"IDO", "SEPIO", "TTO", "PR", "NBO", "WBPHENOTYPE", "PECO", "GAZ", "CIO",
	"INO", "CLAO", "UPA", "NOMEN", "ZFA", "DISDRIV", "CIDO", "KISAO", "MA",
	"PO", "CDNO", "OHD", "VARIO", "AGRO", "DIDEO", "TXPO", "PATO", "HOM", "ECO",
	"ICEO", "DDANAT", "BSPO", "MRO", "PCO", "EPSO", "ORNASEQ", "HP", "DPO",
	"CL", "MFOEM",
]